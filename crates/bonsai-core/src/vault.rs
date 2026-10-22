//! Device-bound account vault. Only ciphertext leaves this module. The keychain stores a
//! separate random secret, never the vault key or a PIN verifier. See docs/account-vault.md.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::{Duration, Instant};

use argon2::{Algorithm, Argon2, Params, Version};
use bonsai_api::{AccountId, Secret};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use fs2::FileExt;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::account::CookieStore;

type Result<T> = std::result::Result<T, String>;
const SERVICE: &str = "io.github.6e6b.bonsai.vault.v1";
const MAX_FILE: u64 = 8 * 1024 * 1024;
const LOCKED: &str = "Unlock Bonsai to access accounts.";
const INVALID: &str = "The PIN is incorrect or the vault could not be authenticated.";

/// Config files cannot disable the timer with zero or create an unbounded timeout.
pub fn idle_timeout(minutes: u32) -> Duration {
    Duration::from_secs(u64::from(minutes.clamp(1, 60)) * 60)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Disabled,
    Locked,
    Unlocked,
    Unavailable,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Cost {
    memory: u32,
    passes: u32,
    lanes: u32,
}
impl Default for Cost {
    fn default() -> Self {
        Self {
            memory: 65536,
            passes: 3,
            lanes: 4,
        }
    }
}
impl Cost {
    fn params(self) -> Result<Params> {
        // Validate BEFORE allocating KDF memory, even though the header is authenticated later.
        if !(65536..=262144).contains(&self.memory) || !(3..=6).contains(&self.passes) || self.lanes != 4 {
            return Err("Unsupported vault KDF parameters.".into());
        }
        Params::new(self.memory, self.passes, self.lanes, Some(32))
            .map_err(|_| "Invalid vault KDF parameters.".into())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Header {
    version: u32,
    id: [u8; 16],
    salt: [u8; 16],
    cost: Cost,
}
#[derive(Clone, Serialize, Deserialize)]
struct Envelope {
    nonce: [u8; 24],
    bytes: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
struct Document {
    header: Header,
    wrapped: Envelope,
    contents: Envelope,
}
#[derive(Default, Serialize, Deserialize)]
struct Contents {
    cookies: BTreeMap<u64, Zeroizing<String>>,
    // Restartable migration: old credentials are only removed after the durable vault exists.
    pending_cleanup: Vec<u64>,
    cache_migrated: bool,
}
#[derive(Serialize, Deserialize)]
struct Device {
    enabled: bool,
    secret: Zeroizing<Vec<u8>>,
}

trait DeviceStore: Send + Sync {
    fn read(&self) -> Result<Option<Device>>;
    fn write(&self, device: &Device) -> Result<()>;
}
struct OsDevice {
    account: String,
}
impl DeviceStore for OsDevice {
    fn read(&self) -> Result<Option<Device>> {
        let e = keyring::Entry::new(SERVICE, &self.account).map_err(|_| "OS keychain unavailable.")?;
        match e.get_password() {
            Ok(raw) => serde_json::from_str(&Zeroizing::new(raw))
                .map(Some)
                .map_err(|_| "Vault device secret is invalid.".into()),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err("OS keychain unavailable. Unlock your system keychain and try again.".into()),
        }
    }
    fn write(&self, device: &Device) -> Result<()> {
        let raw = Zeroizing::new(serde_json::to_string(device).map_err(|_| "Cannot encode device secret.")?);
        let e = keyring::Entry::new(SERVICE, &self.account).map_err(|_| "OS keychain unavailable.")?;
        e.set_password(&raw)
            .map_err(|_| "Cannot save the vault device secret to the OS keychain.")?;
        let saved = self
            .read()?
            .ok_or("OS keychain did not save the device secret.")?;
        if saved.enabled != device.enabled || saved.secret != device.secret {
            return Err("OS keychain verification failed.".into());
        }
        Ok(())
    }
}

struct OpenVault {
    doc: Document,
    key: Zeroizing<[u8; 32]>,
    contents: Contents,
}
enum Mode {
    Disabled,
    Locked,
    Open(Box<OpenVault>),
    Unavailable(String),
}
struct State {
    mode: Mode,
    failures: u32,
    retry: Option<Instant>,
}

pub struct Vault {
    file: PathBuf,
    device: Arc<dyn DeviceStore>,
    unprotected: Arc<dyn CookieStore>,
    state: Mutex<State>,
    blocked: AtomicBool,
    enabled: AtomicBool,
    status: AtomicU8,
    // Held for the process lifetime. Never allow independent read/modify/write vault writers.
    _writer: Option<File>,
}

impl Vault {
    pub fn open(dir: &Path, unprotected: Arc<dyn CookieStore>) -> Self {
        use sha2::{Digest, Sha256};
        let account = Sha256::digest(dir.as_os_str().as_encoded_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        Self::with_device(dir, unprotected, Arc::new(OsDevice { account }))
    }
    fn with_device(dir: &Path, unprotected: Arc<dyn CookieStore>, device: Arc<dyn DeviceStore>) -> Self {
        let file = dir.join("account-vault.json");
        let writer = (|| -> Result<File> {
            std::fs::create_dir_all(dir).map_err(|_| "Cannot create vault directory.")?;
            let mut opts = OpenOptions::new();
            opts.read(true).write(true).create(true).truncate(false);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                opts.mode(0o600);
            }
            let f = opts
                .open(dir.join("account-vault.lock"))
                .map_err(|_| "Cannot open vault process lock.")?;
            f.try_lock_exclusive()
                .map_err(|_| "Another Bonsai process is using this account vault.")?;
            Ok(f)
        })();
        let mode = match &writer {
            Err(e) => Mode::Unavailable(e.clone()),
            Ok(_) => match device.read() {
                Ok(Some(d)) if d.enabled => Mode::Locked,
                Ok(Some(_)) => Mode::Disabled, // Authenticated disable committed before cleanup.
                Ok(None) if !file.exists() => Mode::Disabled,
                Ok(None) => Mode::Locked, // Missing device secret must never bypass an existing vault.
                Err(_) if !file.exists() => Mode::Disabled,
                Err(e) => Mode::Unavailable(e),
            },
        };
        let blocked = !matches!(mode, Mode::Disabled);
        let status = if matches!(mode, Mode::Unavailable(_)) {
            3
        } else if blocked {
            1
        } else {
            0
        };
        Self {
            file,
            device,
            unprotected,
            state: Mutex::new(State {
                mode,
                failures: 0,
                retry: None,
            }),
            blocked: AtomicBool::new(blocked),
            enabled: AtomicBool::new(blocked),
            status: AtomicU8::new(status),
            _writer: writer.ok(),
        }
    }
    pub fn status(&self) -> Status {
        match self.status.load(Ordering::Acquire) {
            0 => Status::Disabled,
            1 => Status::Locked,
            2 => Status::Unlocked,
            _ => Status::Unavailable,
        }
    }
    pub fn blocked(&self) -> bool {
        self.blocked.load(Ordering::Acquire)
    }
    pub fn error(&self) -> Option<String> {
        match self.state.try_lock().as_deref().map(|s| &s.mode) {
            Some(Mode::Unavailable(e)) => Some(e.clone()),
            _ => None,
        }
    }
    pub fn retry_seconds(&self) -> u32 {
        self.state.try_lock().and_then(|s| s.retry).map_or(0, |t| {
            t.saturating_duration_since(Instant::now())
                .as_secs()
                .saturating_add(1) as u32
        })
    }
    pub fn protected(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }
    pub fn close_admission(&self) {
        self.blocked.store(true, Ordering::Release);
    }
    pub fn validate_pin(pin: &str) -> Result<()> {
        let n = pin.chars().count();
        let numeric = pin.bytes().all(|c| c.is_ascii_digit());
        if pin.len() > 256 || n < if numeric { 8 } else { 12 } || pin.trim().is_empty() {
            return Err(
                "Use at least 8 digits or a passphrase of at least 12 characters (maximum 256 bytes).".into(),
            );
        }
        if numeric
            && (pin.bytes().all(|b| b == pin.as_bytes()[0])
                || "01234567890123456789".contains(pin)
                || "98765432109876543210".contains(pin))
        {
            return Err("Choose a less predictable PIN.".into());
        }
        Ok(())
    }
    pub fn enable(&self, pin: &str, accounts: &[AccountId]) -> Result<()> {
        Self::validate_pin(pin)?;
        let mut s = self.state.lock();
        if !matches!(s.mode, Mode::Disabled) {
            return Err("Vault protection is already enabled or unavailable.".into());
        }
        let mut contents = Contents::default();
        for id in accounts {
            if let Some(c) = self.unprotected.get(*id)? {
                contents
                    .cookies
                    .insert(id.0, Zeroizing::new(c.expose().to_owned()));
            }
            contents.pending_cleanup.push(id.0);
        }
        let key = Zeroizing::new(random::<32>()?);
        let header = Header {
            version: 1,
            id: random()?,
            salt: random()?,
            cost: Cost::default(),
        };
        let device = Device {
            enabled: true,
            secret: Zeroizing::new(random::<32>()?.to_vec()),
        };
        let kek = derive(pin, &device, &header)?;
        let wrapped = seal(&kek, &header_aad(&header)?, key.as_slice())?;
        let encrypted =
            Zeroizing::new(serde_json::to_vec(&contents).map_err(|_| "Cannot encode vault contents.")?);
        let doc = Document {
            contents: seal(&key, &payload_aad(&header), &encrypted)?,
            header,
            wrapped,
        };
        // From this point any failure leaves protection enabled and closed, never a fallback.
        self.blocked.store(true, Ordering::Release);
        self.enabled.store(true, Ordering::Release);
        self.status.store(1, Ordering::Release);
        s.mode = Mode::Locked;
        self.device.write(&device)?;
        save(&self.file, &doc)?;
        // Verify the persisted bytes before deleting any source credential.
        let verified = read_document(&self.file)?;
        let unwrapped = open(&kek, &header_aad(&verified.header)?, &verified.wrapped)?;
        if unwrapped.as_slice() != key.as_slice() {
            return Err("Vault verification failed.".into());
        }
        let _ = open(&key, &payload_aad(&verified.header), &verified.contents)?;
        s.mode = Mode::Open(Box::new(OpenVault { doc, key, contents }));
        // Migration is not complete yet. The host must encrypt the cache and finish cleanup.
        Ok(())
    }
    pub fn unlock(&self, pin: &str) -> Result<()> {
        if pin.len() > 256 {
            return Err(INVALID.into());
        }
        let mut s = self.state.lock();
        if let Mode::Unavailable(e) = &s.mode {
            return Err(e.clone());
        }
        if matches!(s.mode, Mode::Disabled) {
            return Err("Vault protection is disabled.".into());
        }
        if matches!(s.mode, Mode::Open(_)) {
            return Ok(());
        }
        if let Some(t) = s.retry
            && t > Instant::now()
        {
            return Err("Too many attempts. Wait before trying again.".into());
        }
        let result = self.authenticate(pin);
        match result {
            Ok(v) => {
                s.mode = Mode::Open(Box::new(v));
                s.failures = 0;
                s.retry = None;
                Ok(())
            }
            Err(e) => {
                s.failures = s.failures.saturating_add(1);
                // This is UI abuse protection, not an offline security boundary.
                if s.failures >= 3 {
                    s.retry =
                        Some(Instant::now() + Duration::from_secs((1u64 << s.failures.min(8)).min(300)));
                }
                Err(e)
            }
        }
    }
    fn authenticate(&self, pin: &str) -> Result<OpenVault> {
        let device = self.device.read()?.filter(|d| d.enabled).ok_or(
            "The vault device secret is missing. Restore it or sign in again in a new Bonsai profile.",
        )?;
        let doc = read_document(&self.file)?;
        let kek = derive(pin, &device, &doc.header)?;
        let bytes = open(&kek, &header_aad(&doc.header)?, &doc.wrapped)?;
        let mut key = Zeroizing::new([0u8; 32]);
        if bytes.len() != key.len() {
            return Err(INVALID.into());
        }
        key.copy_from_slice(&bytes);
        let raw = open(&key, &payload_aad(&doc.header), &doc.contents)?;
        let contents: Contents = serde_json::from_slice(&raw).map_err(|_| INVALID)?;
        Ok(OpenVault { doc, key, contents })
    }
    /// Open access only after both migration steps succeeded.
    pub fn finish_migration(&self) -> Result<()> {
        let mut s = self.state.lock();
        let Mode::Open(v) = &mut s.mode else {
            return Err(LOCKED.into());
        };
        for id in &v.contents.pending_cleanup {
            self.unprotected.delete(AccountId(*id))?;
        }
        v.contents.pending_cleanup.clear();
        v.contents.cache_migrated = true;
        persist(&self.file, v)?;
        self.status.store(2, Ordering::Release);
        self.blocked.store(false, Ordering::Release);
        Ok(())
    }
    pub fn migration_pending(&self) -> bool {
        match &self.state.lock().mode {
            Mode::Open(v) => !v.contents.cache_migrated,
            _ => false,
        }
    }
    /// Close admission immediately. Locking the running app ends the process session; the
    /// shell restarts into Locked, so no old task, webview or plugin VM can survive unlock.
    pub fn lock(&self) {
        self.blocked.store(true, Ordering::Release);
        self.status.store(1, Ordering::Release);
        self.state.lock().mode = Mode::Locked;
    }
    pub fn change_pin(&self, current: &str, next: &str) -> Result<()> {
        Self::validate_pin(next)?;
        let mut s = self.state.lock();
        if !matches!(s.mode, Mode::Open(_)) || self.blocked() {
            return Err(LOCKED.into());
        }
        let verified = self.authenticate(current)?;
        let Mode::Open(v) = &mut s.mode else {
            return Err(LOCKED.into());
        };
        if verified.key != v.key {
            return Err(INVALID.into());
        }
        let device = self.device.read()?.ok_or("Vault device secret missing.")?;
        let mut doc = v.doc.clone();
        doc.header.salt = random()?;
        let kek = derive(next, &device, &doc.header)?;
        doc.wrapped = seal(&kek, &header_aad(&doc.header)?, v.key.as_slice())?;
        // Payload AAD excludes salt/KDF, so PIN changes only rewrap the vault key.
        save(&self.file, &doc)?;
        v.doc = doc;
        Ok(())
    }
    /// Called after the host decrypted the private cache under exclusive admission.
    pub fn disable(&self, current: &str) -> Result<()> {
        let mut s = self.state.lock();
        if !matches!(s.mode, Mode::Open(_)) {
            return Err(LOCKED.into());
        }
        let verified = self.authenticate(current)?;
        let Mode::Open(v) = &mut s.mode else {
            return Err(LOCKED.into());
        };
        if verified.key != v.key {
            return Err(INVALID.into());
        }
        for (id, c) in &v.contents.cookies {
            self.unprotected
                .set_persistent(AccountId(*id), &Secret::new(c.as_str()))?;
        }
        // Commit disable last. Interrupted cleanup is harmless; ciphertext can remain.
        let mut device = self.device.read()?.ok_or("Vault device secret missing.")?;
        device.enabled = false;
        self.device.write(&device)?;
        s.mode = Mode::Disabled;
        self.enabled.store(false, Ordering::Release);
        self.status.store(0, Ordering::Release);
        self.blocked.store(false, Ordering::Release);
        let _ = std::fs::remove_file(&self.file);
        Ok(())
    }
    /// Journal the transition before any cache payload becomes plaintext. A crash leaves
    /// admission closed, and the next unlock re-encrypts all private rows before opening.
    pub fn prepare_disable(&self, current: &str) -> Result<()> {
        let mut s = self.state.lock();
        let verified = self.authenticate(current)?;
        let Mode::Open(v) = &mut s.mode else {
            return Err(LOCKED.into());
        };
        if verified.key != v.key {
            return Err(INVALID.into());
        }
        self.blocked.store(true, Ordering::Release);
        v.contents.cache_migrated = false;
        v.contents.pending_cleanup = v.contents.cookies.keys().copied().collect();
        persist(&self.file, v)
    }
    /// Remove transitional source credentials before discarding their account index.
    /// This never decrypts the vault and keeps admission closed on any failure.
    pub fn prepare_forget(&self, accounts: &[AccountId]) -> Result<()> {
        let _state = self.state.lock();
        if self._writer.is_none() || !self.protected() {
            return Err("Cannot reset this account vault.".into());
        }
        self.blocked.store(true, Ordering::Release);
        for id in accounts {
            self.unprotected.delete(*id)?;
        }
        Ok(())
    }
    /// Finish destructive recovery after the caller cleared metadata and private cache.
    /// The caller must restart; this process never reopens account access.
    pub fn forget(&self, accounts: &[AccountId]) -> Result<()> {
        let mut state = self.state.lock();
        if self._writer.is_none() || !self.protected() {
            return Err("Cannot reset this account vault.".into());
        }
        self.blocked.store(true, Ordering::Release);
        for id in accounts {
            self.unprotected.delete(*id)?;
        }
        // Destroy the old device secret only after source credentials and all caller-owned
        // account data were removed. A crash before this commit remains locked.
        self.device.write(&Device {
            enabled: false,
            secret: Zeroizing::new(random::<32>()?.to_vec()),
        })?;
        state.mode = Mode::Disabled;
        self.enabled.store(false, Ordering::Release);
        self.status.store(0, Ordering::Release);
        let _ = std::fs::remove_file(&self.file);
        // Admission deliberately remains closed until a fresh process starts.
        Ok(())
    }
    /// AAD is the caller's complete cache key, including table and account, never file paths.
    pub fn encrypt_cache(&self, aad: &[u8], bytes: &[u8]) -> Result<String> {
        use base64::Engine;
        let s = self.state.lock();
        let Mode::Open(v) = &s.mode else {
            return Err(LOCKED.into());
        };
        let mut domain = b"bonsai-cache-v1:".to_vec();
        domain.extend_from_slice(aad);
        let e = seal(&v.key, &domain, bytes)?;
        let raw = serde_json::to_vec(&e).map_err(|_| "Cannot encode encrypted cache.")?;
        Ok(format!(
            "vault1:{}",
            base64::engine::general_purpose::STANDARD.encode(raw)
        ))
    }
    pub fn decrypt_cache(&self, aad: &[u8], text: &str) -> Result<Zeroizing<Vec<u8>>> {
        use base64::Engine;
        let s = self.state.lock();
        let Mode::Open(v) = &s.mode else {
            return Err(LOCKED.into());
        };
        let Some(encoded) = text.strip_prefix("vault1:") else {
            return Err("Unencrypted private cache row rejected.".into());
        };
        let raw = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|_| "Invalid encrypted cache.")?;
        let e = serde_json::from_slice(&raw).map_err(|_| "Invalid encrypted cache.")?;
        let mut domain = b"bonsai-cache-v1:".to_vec();
        domain.extend_from_slice(aad);
        open(&v.key, &domain, &e)
    }
}

impl CookieStore for Vault {
    fn get(&self, id: AccountId) -> Result<Option<Secret>> {
        let s = self.state.lock();
        match &s.mode {
            Mode::Disabled if !self.blocked() => self.unprotected.get(id),
            Mode::Open(v) if !self.blocked() => {
                Ok(v.contents.cookies.get(&id.0).map(|s| Secret::new(s.as_str())))
            }
            _ => Err(LOCKED.into()),
        }
    }
    fn set(&self, id: AccountId, cookie: &Secret) -> Result<()> {
        let mut s = self.state.lock();
        match &mut s.mode {
            Mode::Disabled if !self.blocked() => self.unprotected.set(id, cookie),
            Mode::Open(v) if !self.blocked() => {
                let previous = v
                    .contents
                    .cookies
                    .insert(id.0, Zeroizing::new(cookie.expose().to_owned()));
                if let Err(e) = persist(&self.file, v) {
                    match previous {
                        Some(c) => {
                            v.contents.cookies.insert(id.0, c);
                        }
                        None => {
                            v.contents.cookies.remove(&id.0);
                        }
                    }
                    return Err(e);
                }
                Ok(())
            }
            _ => Err(LOCKED.into()),
        }
    }
    fn delete(&self, id: AccountId) -> Result<()> {
        let mut s = self.state.lock();
        match &mut s.mode {
            Mode::Disabled if !self.blocked() => self.unprotected.delete(id),
            Mode::Open(v) if !self.blocked() => {
                let previous = v.contents.cookies.remove(&id.0);
                if let Err(e) = persist(&self.file, v) {
                    if let Some(c) = previous {
                        v.contents.cookies.insert(id.0, c);
                    }
                    return Err(e);
                }
                Ok(())
            }
            _ => Err(LOCKED.into()),
        }
    }
}
fn random<const N: usize>() -> Result<[u8; N]> {
    let mut bytes = [0u8; N];
    getrandom::fill(&mut bytes).map_err(|_| "OS random generator unavailable.")?;
    Ok(bytes)
}
fn derive(pin: &str, device: &Device, h: &Header) -> Result<Zeroizing<[u8; 32]>> {
    if h.version != 1 || device.secret.len() != 32 {
        return Err("Unsupported vault or invalid device secret.".into());
    }
    let params = h.cost.params()?;
    let argon = Argon2::new_with_secret(&device.secret, Algorithm::Argon2id, Version::V0x13, params)
        .map_err(|_| "Cannot initialize vault KDF.")?;
    let mut key = Zeroizing::new([0u8; 32]);
    argon
        .hash_password_into(pin.as_bytes(), &h.salt, key.as_mut())
        .map_err(|_| "Cannot derive vault key.")?;
    Ok(key)
}
fn header_aad(h: &Header) -> Result<Vec<u8>> {
    serde_json::to_vec(&("bonsai-vault-key-v1", h)).map_err(|_| "Invalid vault header.".into())
}
fn payload_aad(h: &Header) -> Vec<u8> {
    let mut aad = b"bonsai-vault-contents-v1:".to_vec();
    aad.extend_from_slice(&h.id);
    aad
}
fn seal(key: &[u8; 32], aad: &[u8], bytes: &[u8]) -> Result<Envelope> {
    let nonce = random()?;
    let bytes = XChaCha20Poly1305::new(key.into())
        .encrypt(XNonce::from_slice(&nonce), Payload { msg: bytes, aad })
        .map_err(|_| "Vault encryption failed.")?;
    Ok(Envelope { nonce, bytes })
}
fn open(key: &[u8; 32], aad: &[u8], e: &Envelope) -> Result<Zeroizing<Vec<u8>>> {
    XChaCha20Poly1305::new(key.into())
        .decrypt(XNonce::from_slice(&e.nonce), Payload { msg: &e.bytes, aad })
        .map(Zeroizing::new)
        .map_err(|_| INVALID.into())
}
fn persist(path: &Path, v: &mut OpenVault) -> Result<()> {
    let raw = Zeroizing::new(serde_json::to_vec(&v.contents).map_err(|_| "Cannot encode vault contents.")?);
    let contents = seal(&v.key, &payload_aad(&v.doc.header), &raw)?;
    let doc = Document {
        contents,
        ..v.doc.clone()
    };
    save(path, &doc)?;
    v.doc = doc;
    Ok(())
}
fn read_document(path: &Path) -> Result<Document> {
    let f = File::open(path).map_err(|_| "Cannot open the account vault. Protection remains enabled.")?;
    let mut bytes = Vec::new();
    f.take(MAX_FILE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read account vault.")?;
    if bytes.len() as u64 > MAX_FILE {
        return Err("Account vault exceeds its size limit.".into());
    }
    serde_json::from_slice(&bytes).map_err(|_| "Account vault is damaged. Protection remains enabled.".into())
}
fn save(path: &Path, doc: &Document) -> Result<()> {
    let bytes = serde_json::to_vec(doc).map_err(|_| "Cannot encode account vault.")?;
    if bytes.len() as u64 > MAX_FILE {
        return Err("Account vault exceeds its size limit.".into());
    }
    let dir = path.parent().ok_or("Invalid vault directory.")?;
    let mut tmp = tempfile::NamedTempFile::new_in(dir).map_err(|_| "Cannot create encrypted vault file.")?;
    tmp.write_all(&bytes)
        .and_then(|_| tmp.as_file().sync_all())
        .map_err(|_| "Cannot write encrypted vault file.")?;
    tmp.persist(path)
        .map_err(|_| "Cannot replace encrypted vault file.")?;
    #[cfg(unix)]
    File::open(dir)
        .and_then(|d| d.sync_all())
        .map_err(|_| "Cannot sync vault directory.")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::MemoryCookies;
    use std::sync::atomic::AtomicBool;
    const PIN: &str = "59371482";
    const NEXT: &str = "a long test passphrase";
    #[derive(Default)]
    struct MemoryDevice {
        raw: Mutex<Option<Zeroizing<String>>>,
        fail: AtomicBool,
    }
    impl DeviceStore for MemoryDevice {
        fn read(&self) -> Result<Option<Device>> {
            self.raw
                .lock()
                .as_ref()
                .map(|s| serde_json::from_str(s).map_err(|_| "bad device".into()))
                .transpose()
        }
        fn write(&self, d: &Device) -> Result<()> {
            if self.fail.load(Ordering::Relaxed) {
                return Err("keychain failure".into());
            }
            *self.raw.lock() = Some(Zeroizing::new(serde_json::to_string(d).unwrap()));
            Ok(())
        }
    }
    struct Fixture {
        dir: tempfile::TempDir,
        unprotected: Arc<MemoryCookies>,
        device: Arc<MemoryDevice>,
    }
    impl Fixture {
        fn new() -> Self {
            Self {
                dir: tempfile::tempdir().unwrap(),
                unprotected: Arc::new(MemoryCookies::default()),
                device: Arc::new(MemoryDevice::default()),
            }
        }
        fn vault(&self) -> Vault {
            Vault::with_device(self.dir.path(), self.unprotected.clone(), self.device.clone())
        }
        fn file(&self) -> PathBuf {
            self.dir.path().join("account-vault.json")
        }
        fn enable(&self) -> Vault {
            self.unprotected
                .set(AccountId(7), &Secret::new("synthetic-vault-account-seven"))
                .unwrap();
            self.unprotected
                .set(AccountId(9), &Secret::new("synthetic-vault-account-nine"))
                .unwrap();
            let v = self.vault();
            v.enable(PIN, &[AccountId(7), AccountId(9)]).unwrap();
            v.finish_migration().unwrap();
            v
        }
    }
    #[test]
    fn one_secret_unlocks_every_account_after_restart() {
        let f = Fixture::new();
        let v = f.enable();
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_none());
        assert!(f.unprotected.get(AccountId(9)).unwrap().is_none());
        drop(v);
        let v = f.vault();
        assert_eq!(v.status(), Status::Locked);
        assert!(v.get(AccountId(7)).is_err());
        v.unlock(PIN).unwrap();
        assert!(v.blocked());
        v.finish_migration().unwrap();
        assert_eq!(
            v.get(AccountId(7)).unwrap().unwrap().expose(),
            "synthetic-vault-account-seven"
        );
        assert_eq!(
            v.get(AccountId(9)).unwrap().unwrap().expose(),
            "synthetic-vault-account-nine"
        );
        let bytes = std::fs::read_to_string(f.file()).unwrap();
        assert!(!bytes.contains("synthetic-vault"));
        assert!(!bytes.contains(PIN));
    }
    #[test]
    fn wrong_pin_and_missing_device_never_fall_back_to_keychain() {
        let f = Fixture::new();
        drop(f.enable());
        f.unprotected
            .set(AccountId(7), &Secret::new("unprotected-bypass"))
            .unwrap();
        let v = f.vault();
        assert!(v.unlock("00000000").is_err());
        assert!(v.get(AccountId(7)).is_err());
        *f.device.raw.lock() = None;
        assert!(v.unlock(PIN).is_err());
        assert!(v.get(AccountId(7)).is_err());
    }
    #[test]
    fn deleted_and_corrupt_vault_remain_locked() {
        let f = Fixture::new();
        drop(f.enable());
        std::fs::remove_file(f.file()).unwrap();
        let v = f.vault();
        assert_eq!(v.status(), Status::Locked);
        assert!(v.unlock(PIN).is_err());
        drop(v);
        std::fs::write(f.file(), b"{broken").unwrap();
        let v = f.vault();
        assert_eq!(v.status(), Status::Locked);
        assert!(v.unlock(PIN).is_err());
    }
    #[test]
    fn another_process_cannot_open_a_second_writer() {
        let f = Fixture::new();
        let first = f.vault();
        let second = f.vault();
        assert_eq!(second.status(), Status::Unavailable);
        assert!(second.blocked());
        drop(first);
        drop(second);
        assert_eq!(f.vault().status(), Status::Disabled);
    }
    #[test]
    fn second_writer_cannot_unlock_or_modify_a_protected_vault() {
        let f = Fixture::new();
        let first = f.enable();
        let second = f.vault();
        assert_eq!(second.status(), Status::Unavailable);
        assert!(second.unlock(PIN).is_err());
        assert!(second.get(AccountId(7)).is_err());
        assert!(second.set(AccountId(7), &Secret::new("second-writer")).is_err());
        assert!(first.get(AccountId(7)).unwrap().is_some());
    }
    #[test]
    fn change_rewraps_key_without_reencrypting_private_records() {
        let f = Fixture::new();
        let v = f.enable();
        let cached = v.encrypt_cache(b"account7", b"private").unwrap();
        let old = read_document(&f.file()).unwrap();
        assert!(v.change_pin("wrong", NEXT).is_err());
        v.change_pin(PIN, NEXT).unwrap();
        let new = read_document(&f.file()).unwrap();
        assert_eq!(old.contents.bytes, new.contents.bytes);
        assert_ne!(old.wrapped.bytes, new.wrapped.bytes);
        drop(v);
        let v = f.vault();
        assert!(v.unlock(PIN).is_err());
        v.unlock(NEXT).unwrap();
        v.finish_migration().unwrap();
        assert_eq!(&*v.decrypt_cache(b"account7", &cached).unwrap(), b"private");
    }
    #[test]
    fn ciphertext_is_bound_to_cache_account_and_key() {
        let f = Fixture::new();
        let v = f.enable();
        let text = v.encrypt_cache(b"account7/item1", b"private-record").unwrap();
        assert!(v.decrypt_cache(b"account9/item1", &text).is_err());
        assert!(v.decrypt_cache(b"account7/item2", &text).is_err());
        assert!(v.decrypt_cache(b"account7/item1", "plaintext-bypass").is_err());
        assert_ne!(
            text,
            v.encrypt_cache(b"account7/item1", b"private-record").unwrap()
        );
    }
    #[test]
    fn header_tampering_and_unbounded_kdf_are_rejected() {
        let f = Fixture::new();
        drop(f.enable());
        let original = read_document(&f.file()).unwrap();
        for field in 0..4 {
            let mut doc = original.clone();
            match field {
                0 => doc.header.salt[0] ^= 1,
                1 => doc.header.id[0] ^= 1,
                2 => doc.header.cost.memory = u32::MAX,
                _ => doc.header.version = 999,
            }
            save(&f.file(), &doc).unwrap();
            let v = f.vault();
            assert!(v.unlock(PIN).is_err());
            assert!(v.blocked());
        }
    }
    #[test]
    fn corrupted_wrapping_and_contents_are_rejected() {
        let f = Fixture::new();
        drop(f.enable());
        let original = read_document(&f.file()).unwrap();
        for field in 0..4 {
            let mut doc = original.clone();
            match field {
                0 => doc.wrapped.bytes[0] ^= 1,
                1 => doc.wrapped.nonce[0] ^= 1,
                2 => doc.contents.bytes[0] ^= 1,
                _ => doc.contents.nonce[0] ^= 1,
            }
            save(&f.file(), &doc).unwrap();
            let v = f.vault();
            assert!(v.unlock(PIN).is_err());
        }
    }
    #[test]
    fn interrupted_enable_retains_source_and_resumes_cleanup() {
        let f = Fixture::new();
        f.unprotected
            .set(AccountId(7), &Secret::new("synthetic-enable-cookie"))
            .unwrap();
        let v = f.vault();
        v.enable(PIN, &[AccountId(7)]).unwrap();
        assert!(v.blocked());
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_some());
        drop(v);
        let v = f.vault();
        v.unlock(PIN).unwrap();
        assert!(v.migration_pending());
        v.finish_migration().unwrap();
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_none());
        assert!(!v.blocked());
    }
    #[test]
    fn unavailable_keychain_does_not_delete_source_credentials() {
        let f = Fixture::new();
        f.unprotected
            .set(AccountId(7), &Secret::new("synthetic-source"))
            .unwrap();
        f.device.fail.store(true, Ordering::Relaxed);
        let v = f.vault();
        assert!(v.enable(PIN, &[AccountId(7)]).is_err());
        assert!(v.blocked());
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_some());
        assert!(!f.file().exists());
    }
    #[test]
    fn failed_atomic_write_keeps_previous_cookie_and_never_removes_source() {
        let f = Fixture::new();
        let v = f.enable();
        let previous = std::fs::read(f.file()).unwrap();
        std::fs::remove_file(f.file()).unwrap();
        std::fs::create_dir(f.file()).unwrap();
        assert!(v.set(AccountId(7), &Secret::new("new-cookie-not-saved")).is_err());
        assert_eq!(
            v.get(AccountId(7)).unwrap().unwrap().expose(),
            "synthetic-vault-account-seven"
        );
        std::fs::remove_dir(f.file()).unwrap();
        std::fs::write(f.file(), previous).unwrap();
        drop(v);
        let v = f.vault();
        v.unlock(PIN).unwrap();
        v.finish_migration().unwrap();
        assert_eq!(
            v.get(AccountId(7)).unwrap().unwrap().expose(),
            "synthetic-vault-account-seven"
        );
    }
    #[test]
    fn disable_requires_pin_and_restores_every_account() {
        let f = Fixture::new();
        let v = f.enable();
        assert!(v.disable("wrong").is_err());
        assert!(v.protected());
        v.prepare_disable(PIN).unwrap();
        v.disable(PIN).unwrap();
        assert!(!v.protected());
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_some());
        assert!(f.unprotected.get(AccountId(9)).unwrap().is_some());
        drop(v);
        assert_eq!(f.vault().status(), Status::Disabled);
    }
    #[test]
    fn lock_drops_unwrapped_keys_and_refuses_reads_and_writes() {
        let f = Fixture::new();
        let v = f.enable();
        v.lock();
        assert!(v.get(AccountId(7)).is_err());
        assert!(v.set(AccountId(7), &Secret::new("anything")).is_err());
        assert!(v.delete(AccountId(7)).is_err());
        assert!(v.encrypt_cache(b"a", b"b").is_err());
    }
    #[test]
    fn failed_guesses_get_a_cooldown() {
        let f = Fixture::new();
        let v = f.enable();
        v.lock();
        for _ in 0..3 {
            assert!(v.unlock("wrong").is_err());
        }
        assert!(v.retry_seconds() > 0);
        assert!(v.unlock(PIN).is_err());
    }
    #[test]
    fn validates_pin_without_trimming_or_reducing_entropy() {
        for value in ["", "1234", "12345678", "88888888", "short words"] {
            assert!(Vault::validate_pin(value).is_err(), "{value}");
        }
        for value in [PIN, NEXT, " space matters "] {
            assert!(Vault::validate_pin(value).is_ok());
        }
        assert!(Vault::validate_pin(&"a".repeat(257)).is_err());
    }
    #[test]
    fn private_sqlite_and_wal_are_encrypted_and_shared_rows_still_work() {
        use crate::cache::{Cache, Kind};
        let f = Fixture::new();
        let path = f.dir.path().join("cache.sqlite3");
        let cache = Cache::open(&path).unwrap();
        cache.put(Kind::Local, AccountId(7), "note", &"secret-private-cache-text");
        cache.put(Kind::User, AccountId(7), "public", &"public-cache-text");
        cache.ledger_put(
            AccountId(7),
            "personal",
            [("purchase", "1", 100, &"secret-ledger-text")],
        );
        let v = Arc::new(f.vault());
        cache.attach_vault(v.clone());
        v.enable(PIN, &[]).unwrap();
        cache.migrate_vault(&v, true).unwrap();
        v.finish_migration().unwrap();
        assert_eq!(
            cache
                .get::<String>(Kind::Local, AccountId(7), "note")
                .unwrap()
                .value,
            "secret-private-cache-text"
        );
        assert_eq!(
            cache.ledger_all::<String>(AccountId(7), "personal"),
            vec!["secret-ledger-text"]
        );
        cache.put(Kind::Local, AccountId(9), "note", &"other-account-text");
        assert_eq!(
            cache
                .get::<String>(Kind::Local, AccountId(7), "note")
                .unwrap()
                .value,
            "secret-private-cache-text"
        );
        for suffix in ["", "-wal"] {
            let bytes = std::fs::read(format!("{}{suffix}", path.display())).unwrap_or_default();
            let text = String::from_utf8_lossy(&bytes);
            assert!(!text.contains("secret-private-cache-text"));
            assert!(!text.contains("secret-ledger-text"));
            assert!(!text.contains("other-account-text"));
        }
        v.lock();
        assert!(cache.get::<String>(Kind::Local, AccountId(7), "note").is_none());
        assert!(cache.ledger_all::<String>(AccountId(7), "personal").is_empty());
        assert_eq!(
            cache
                .get::<String>(Kind::User, AccountId(7), "public")
                .unwrap()
                .value,
            "public-cache-text"
        );
    }
    #[test]
    fn cache_migration_resumes_after_restart_and_never_double_encrypts() {
        use crate::cache::{Cache, Kind};
        let f = Fixture::new();
        let path = f.dir.path().join("cache.sqlite3");
        {
            let cache = Cache::open(&path).unwrap();
            cache.put(Kind::Local, AccountId(7), "note", &"retain-local-note");
            let v = f.vault();
            v.enable(PIN, &[]).unwrap();
            cache.migrate_vault(&v, true).unwrap();
        }
        let cache = Cache::open(&path).unwrap();
        let v = Arc::new(f.vault());
        cache.attach_vault(v.clone());
        v.unlock(PIN).unwrap();
        cache.migrate_vault(&v, true).unwrap();
        v.finish_migration().unwrap();
        assert_eq!(
            cache
                .get::<String>(Kind::Local, AccountId(7), "note")
                .unwrap()
                .value,
            "retain-local-note"
        );
    }
    #[test]
    fn interrupted_disable_reencrypts_cache_before_admission() {
        use crate::cache::{Cache, Kind};
        let f = Fixture::new();
        let path = f.dir.path().join("cache.sqlite3");
        {
            let v = Arc::new(f.enable());
            let cache = Cache::open(&path).unwrap();
            cache.attach_vault(v.clone());
            cache.put(Kind::Local, AccountId(7), "note", &"retain-after-interruption");
            v.prepare_disable(PIN).unwrap();
            cache.migrate_vault(&v, false).unwrap();
            assert!(v.blocked());
            assert!(cache.get::<String>(Kind::Local, AccountId(7), "note").is_none());
            // Crash before the keychain's disable marker is committed.
        }
        let v = Arc::new(f.vault());
        let cache = Cache::open(&path).unwrap();
        cache.attach_vault(v.clone());
        v.unlock(PIN).unwrap();
        assert!(v.migration_pending());
        assert!(v.blocked());
        cache.migrate_vault(&v, true).unwrap();
        v.finish_migration().unwrap();
        assert_eq!(
            cache
                .get::<String>(Kind::Local, AccountId(7), "note")
                .unwrap()
                .value,
            "retain-after-interruption"
        );
        assert!(!v.migration_pending());
        v.lock();
        assert!(cache.get::<String>(Kind::Local, AccountId(7), "note").is_none());
    }
    #[test]
    fn failed_disable_commit_remains_protected_after_restart() {
        let f = Fixture::new();
        let v = f.enable();
        v.prepare_disable(PIN).unwrap();
        f.device.fail.store(true, Ordering::Relaxed);
        assert!(v.disable(PIN).is_err());
        assert!(v.protected());
        assert!(v.blocked());
        drop(v);
        f.device.fail.store(false, Ordering::Relaxed);
        let v = f.vault();
        assert_eq!(v.status(), Status::Locked);
        assert!(v.get(AccountId(7)).is_err());
        v.unlock(PIN).unwrap();
        v.finish_migration().unwrap();
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_none());
    }
    #[test]
    fn recovery_destroys_credentials_without_decrypting_or_reopening_this_process() {
        use crate::cache::{Cache, Kind};
        let f = Fixture::new();
        let v = Arc::new(f.enable());
        let cache = Cache::in_memory().unwrap();
        cache.attach_vault(v.clone());
        cache.put(Kind::Local, AccountId(7), "note", &"private-note");
        cache.put(Kind::User, AccountId(7), "public", &"public-user");
        cache.ledger_put(
            AccountId(7),
            "personal",
            [("purchase", "1", 100, &"private-ledger")],
        );
        v.lock();
        f.unprotected
            .set(AccountId(7), &Secret::new("interrupted-disable-source"))
            .unwrap();
        v.prepare_forget(&[AccountId(7), AccountId(9)]).unwrap();
        cache.forget_private().unwrap();
        v.forget(&[AccountId(7), AccountId(9)]).unwrap();
        assert!(!v.protected());
        assert!(v.blocked());
        assert!(v.unlock(PIN).is_err());
        assert!(v.get(AccountId(7)).is_err());
        assert!(v.set(AccountId(7), &Secret::new("late-signin")).is_err());
        assert!(f.unprotected.get(AccountId(7)).unwrap().is_none());
        // Late private results cannot repopulate reset data before the restart.
        cache.put(Kind::Local, AccountId(7), "late", &"late-private-data");
        assert!(cache.get::<String>(Kind::Local, AccountId(7), "late").is_none());
        assert!(cache.ledger_all::<String>(AccountId(7), "personal").is_empty());
        drop(cache);
        drop(v);
        assert_eq!(f.vault().status(), Status::Disabled);
    }
    #[test]
    fn failed_recovery_commit_stays_locked_and_can_be_retried() {
        let f = Fixture::new();
        let v = f.enable();
        v.lock();
        f.device.fail.store(true, Ordering::Relaxed);
        assert!(v.forget(&[AccountId(7), AccountId(9)]).is_err());
        assert!(v.protected());
        assert!(v.blocked());
        drop(v);
        let v = f.vault();
        assert_eq!(v.status(), Status::Locked);
        f.device.fail.store(false, Ordering::Relaxed);
        v.forget(&[AccountId(7), AccountId(9)]).unwrap();
        assert!(v.blocked());
    }
    #[test]
    fn inactivity_timeout_cannot_be_disabled_or_made_unbounded_by_config() {
        assert_eq!(idle_timeout(0), Duration::from_secs(60));
        assert_eq!(idle_timeout(5), Duration::from_secs(300));
        assert_eq!(idle_timeout(u32::MAX), Duration::from_secs(3600));
    }
    fn core_in(dir: &Path) -> crate::Core {
        use crate::launcher::RecordingOpener;
        use crate::{Core, CoreConfig, Paths};
        let paths = Paths::in_dir(dir);
        crate::account::AccountList {
            accounts: vec![crate::account::AccountMeta {
                id: AccountId(7),
                ..Default::default()
            }],
            active: Some(AccountId(7)),
            ..Default::default()
        }
        .save(&paths.accounts_file())
        .unwrap();
        let cookies = Arc::new(MemoryCookies::default());
        cookies
            .set(AccountId(7), &Secret::new("synthetic-existing-session"))
            .unwrap();
        Core::start(
            CoreConfig {
                paths,
                base: bonsai_api::Base::Replay("http://127.0.0.1:9".into()),
                cookies,
                opener: Arc::new(RecordingOpener::default()),
                limiter: Arc::new(bonsai_api::RateLimiter::default()),
                background: false,
                quick_poll: Duration::from_secs(5),
            },
            Arc::new(|| {}),
        )
    }
    #[test]
    fn invalid_setup_secret_does_not_revoke_existing_clients() {
        let dir = tempfile::tempdir().unwrap();
        let core = core_in(dir.path());
        assert!(core.inner.client(AccountId(7)).is_some());
        assert!(core.vault_action("enable", "", "1234").is_err());
        assert!(!core.accounts_locked());
        assert!(core.inner.client(AccountId(7)).is_some());
    }
    #[test]
    fn unavailable_persistent_cache_prevents_claiming_encrypted_setup() {
        let dir = tempfile::tempdir().unwrap();
        let paths = crate::Paths::in_dir(dir.path());
        std::fs::create_dir_all(paths.db_file()).unwrap();
        let core = core_in(dir.path());
        let error = core.vault_action("enable", "", PIN).unwrap_err();
        assert!(error.contains("cache"));
        assert!(!core.accounts_locked());
        assert!(core.inner.client(AccountId(7)).is_some());
    }
    #[test]
    fn lock_during_setup_stays_closed_even_if_the_storage_transition_finishes() {
        let dir = tempfile::tempdir().unwrap();
        let core = core_in(dir.path());
        // Model an OS lock after setup started but before its durable enabled marker.
        core.inner.vault_busy.store(true, Ordering::Release);
        assert!(core.close_vault_session());
        assert!(!core.close_vault_session());
        // Setup completion would ordinarily open these two admission flags again.
        core.inner.vault_busy.store(false, Ordering::Release);
        core.inner.vault.blocked.store(false, Ordering::Release);
        assert!(core.accounts_locked());
        assert!(core.vault_action("enable", "", PIN).is_err());
    }
    #[test]
    #[ignore = "requires an unlocked native keychain; uses synthetic test data only"]
    fn real_os_keychain_round_trip() {
        struct Cleanup(String);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                if let Ok(entry) = keyring::Entry::new(SERVICE, &self.0) {
                    let _ = entry.delete_credential();
                }
            }
        }
        let dir = tempfile::tempdir().unwrap();
        let cleanup = Cleanup(format!(
            "vault-test-{}",
            random::<16>()
                .unwrap()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        ));
        let device = Arc::new(OsDevice {
            account: cleanup.0.clone(),
        });
        let cookies = Arc::new(MemoryCookies::default());
        cookies
            .set(AccountId(7), &Secret::new("synthetic-keychain-test"))
            .unwrap();
        let v = Vault::with_device(dir.path(), cookies.clone(), device.clone());
        v.enable(PIN, &[AccountId(7)]).unwrap();
        v.finish_migration().unwrap();
        drop(v);
        let v = Vault::with_device(dir.path(), cookies, device);
        assert_eq!(v.status(), Status::Locked);
        v.unlock(PIN).unwrap();
        v.finish_migration().unwrap();
        assert_eq!(
            v.get(AccountId(7)).unwrap().unwrap().expose(),
            "synthetic-keychain-test"
        );
        v.prepare_disable(PIN).unwrap();
        v.disable(PIN).unwrap();
    }
    #[test]
    #[ignore = "manual KDF timing on the current machine"]
    fn measure_unlock_cost() {
        let f = Fixture::new();
        let v = f.enable();
        let mut times = Vec::new();
        for _ in 0..5 {
            v.lock();
            let start = Instant::now();
            v.unlock(PIN).unwrap();
            times.push(start.elapsed());
            v.finish_migration().unwrap();
        }
        times.sort();
        println!(
            "Argon2id: 64 MiB, 3 passes, 4 lanes; median unlock {:?}",
            times[2]
        );
    }
    #[test]
    fn oversized_file_is_rejected_before_parsing_or_kdf() {
        let f = Fixture::new();
        drop(f.enable());
        File::create(f.file()).unwrap().set_len(MAX_FILE + 1).unwrap();
        let v = f.vault();
        assert!(v.unlock(PIN).unwrap_err().contains("size limit"));
    }
}
