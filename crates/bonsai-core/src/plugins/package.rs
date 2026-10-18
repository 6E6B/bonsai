//! `.bonsaiplugin` packages: a zip of a plugin folder, and a content hash that names exactly
//! which files it holds. Reading refuses anything that could leave the folder or fill the disk
//! (absolute paths, `..`, symlinks, encrypted entries, more than the install caps) and skips
//! dot files, like installing from a folder. Only stored and deflated entries are read, which
//! covers every common zip tool; packages this module writes are deflated.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::{MANIFEST, MAX_BYTES, MAX_DEPTH, MAX_FILES, walk};

/// The file extension of a plugin package.
pub const EXTENSION: &str = "bonsaiplugin";

/// One file in a plugin: its path inside the folder, with `/` between parts, and its bytes.
pub(super) type Files = Vec<(String, Vec<u8>)>;

/// SHA-256 over each file in path order: its path, a zero byte, its length as 8 little-endian
/// bytes and its contents. The same for a folder and for a package of it, since both skip dot
/// files.
pub fn content_hash(files: &Files) -> String {
    let mut sorted: Vec<&(String, Vec<u8>)> = files.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut h = Sha256::new();
    for (path, bytes) in sorted {
        h.update(path.as_bytes());
        h.update([0]);
        h.update((bytes.len() as u64).to_le_bytes());
        h.update(bytes);
    }
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn too_large() -> String {
    format!(
        "The plugin is too large: at most {MAX_FILES} files and {} MB.",
        MAX_BYTES / 1024 / 1024
    )
}

/// Reads every file a plugin folder would install: no dot files or symlinks, within the caps.
pub(super) fn read_folder(dir: &Path) -> Result<Files, String> {
    let mut paths = Vec::new();
    let mut bytes = 0;
    let mut over = false;
    walk(dir, 0, &mut |rel, meta| {
        bytes += meta.len();
        paths.push(rel.to_path_buf());
        over = paths.len() > MAX_FILES || bytes > MAX_BYTES;
        !over
    });
    if over {
        return Err(too_large());
    }
    paths
        .into_iter()
        .map(|rel| {
            let name: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            let data =
                std::fs::read(dir.join(&rel)).map_err(|e| format!("Can't read {}: {e}", rel.display()))?;
            Ok((name.join("/"), data))
        })
        .collect()
}

/// A path inside a package, checked part by part. `None` skips it (folders and dot files).
fn entry_path(name: &str) -> Result<Option<String>, String> {
    let bad = || {
        Err(format!(
            "The package has a file outside the plugin folder: \"{name}\"."
        ))
    };
    if name.ends_with('/') {
        return Ok(None);
    }
    if name.is_empty() || name.starts_with('/') || name.contains('\\') || name.contains('\0') {
        return bad();
    }
    let parts: Vec<&str> = name.split('/').collect();
    if parts.len() > MAX_DEPTH + 2 {
        return bad();
    }
    for p in &parts {
        if p.is_empty() || *p == "." || *p == ".." || p.contains(':') {
            return bad();
        }
    }
    if parts.iter().any(|p| p.starts_with('.')) {
        return Ok(None);
    }
    Ok(Some(name.to_owned()))
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

const LOCAL: u32 = 0x0403_4b50;
const CENTRAL: u32 = 0x0201_4b50;
const END: u32 = 0x0605_4b50;

/// Reads the files in a zip. Sizes come from inflating, not from what the headers claim.
pub(super) fn read_zip(zip: &[u8]) -> Result<Files, String> {
    let broken = || "That isn't a plugin package, or it is damaged.".to_owned();
    // The end record is the last 22 bytes plus a comment of up to 64 KB.
    let lowest = zip.len().saturating_sub(22 + 0xffff);
    let end = (lowest..=zip.len().saturating_sub(22))
        .rev()
        .find(|&i| u32_at(zip, i) == Some(END))
        .ok_or_else(broken)?;
    let count = usize::from(u16_at(zip, end + 10).ok_or_else(broken)?);
    let mut at = u32_at(zip, end + 16).ok_or_else(broken)? as usize;
    if count > MAX_FILES * 2 {
        return Err(too_large());
    }
    let mut files: Files = Vec::new();
    let mut total: u64 = 0;
    for _ in 0..count {
        if u32_at(zip, at) != Some(CENTRAL) {
            return Err(broken());
        }
        let field = |off: usize| u16_at(zip, at + off).ok_or_else(broken);
        let made_by = field(4)?;
        let flags = field(8)?;
        let method = field(10)?;
        let crc = u32_at(zip, at + 16).ok_or_else(broken)?;
        let packed_size = u32_at(zip, at + 20).ok_or_else(broken)? as usize;
        let size = u64::from(u32_at(zip, at + 24).ok_or_else(broken)?);
        let (name_len, extra_len, comment_len) = (
            usize::from(field(28)?),
            usize::from(field(30)?),
            usize::from(field(32)?),
        );
        let attrs = u32_at(zip, at + 38).ok_or_else(broken)?;
        let offset = u32_at(zip, at + 42).ok_or_else(broken)? as usize;
        let raw_name = zip.get(at + 46..at + 46 + name_len).ok_or_else(broken)?;
        at += 46 + name_len + extra_len + comment_len;
        let name = std::str::from_utf8(raw_name)
            .map_err(|_| "The package has a file name that isn't UTF-8.".to_owned())?;
        // Unix symlinks keep their mode in the high bits of the external attributes.
        if made_by >> 8 == 3 && (attrs >> 16) & 0o170_000 == 0o120_000 {
            return Err(format!("The package has a symlink: \"{name}\"."));
        }
        let Some(path) = entry_path(name)? else {
            continue;
        };
        if flags & 1 != 0 {
            return Err(format!("\"{name}\" is encrypted."));
        }
        if files.iter().any(|(p, _)| *p == path) {
            return Err(format!("The package lists \"{name}\" twice."));
        }
        if files.len() >= MAX_FILES || total + size > MAX_BYTES {
            return Err(too_large());
        }
        if u32_at(zip, offset) != Some(LOCAL) {
            return Err(broken());
        }
        let local_name = usize::from(u16_at(zip, offset + 26).ok_or_else(broken)?);
        let local_extra = usize::from(u16_at(zip, offset + 28).ok_or_else(broken)?);
        let start = offset + 30 + local_name + local_extra;
        let packed = zip.get(start..start + packed_size).ok_or_else(broken)?;
        // Inflate at most one byte more than is left, so a bomb stops at the cap.
        let room = MAX_BYTES - total;
        let data = match method {
            0 => packed.to_vec(),
            8 => {
                let mut out = Vec::new();
                flate2::read::DeflateDecoder::new(packed)
                    .take(room + 1)
                    .read_to_end(&mut out)
                    .map_err(|_| broken())?;
                out
            }
            _ => return Err(format!("\"{name}\" is compressed in a way Bonsai can't read.")),
        };
        if data.len() as u64 > room {
            return Err(too_large());
        }
        if data.len() as u64 != size {
            return Err(broken());
        }
        let mut c = flate2::Crc::new();
        c.update(&data);
        if c.sum() != crc {
            return Err(broken());
        }
        total += size;
        files.push((path, data));
    }
    Ok(strip_folder(files))
}

/// A zip made by compressing the plugin folder itself holds `<folder>/plugin.toml`; drop that
/// one folder so the manifest is at the top.
fn strip_folder(files: Files) -> Files {
    if files.iter().any(|(p, _)| p == MANIFEST) {
        return files;
    }
    let Some(top) = files
        .first()
        .and_then(|(p, _)| p.split_once('/'))
        .map(|(t, _)| format!("{t}/"))
    else {
        return files;
    };
    let shared = files.iter().all(|(p, _)| p.starts_with(&top));
    if !shared || !files.iter().any(|(p, _)| p.strip_prefix(&top) == Some(MANIFEST)) {
        return files;
    }
    files
        .into_iter()
        .map(|(p, d)| (p[top.len()..].to_owned(), d))
        .collect()
}

/// Writes files as a zip, deflated, in path order with fixed times, so the same folder always
/// packs to the same bytes.
pub(super) fn write_zip(files: &Files) -> Result<Vec<u8>, String> {
    let mut sorted: Vec<&(String, Vec<u8>)> = files.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out: Vec<u8> = Vec::new();
    let mut central: Vec<u8> = Vec::new();
    // 1980-01-01 00:00, the earliest DOS date.
    let (time, date): (u16, u16) = (0, (1 << 5) | 1);
    let small = |n: usize| u32::try_from(n).map_err(|_| too_large());
    for (path, data) in &sorted {
        let mut enc = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        enc.write_all(data).map_err(|e| e.to_string())?;
        let packed = enc.finish().map_err(|e| e.to_string())?;
        let mut c = flate2::Crc::new();
        c.update(data);
        let name = path.as_bytes();
        let name_len = u16::try_from(name.len()).map_err(|_| format!("\"{path}\" has too long a name."))?;
        let offset = small(out.len())?;
        // Version 2.0, UTF-8 names, deflate.
        let common = |b: &mut Vec<u8>| -> Result<(), String> {
            b.extend_from_slice(&20u16.to_le_bytes());
            b.extend_from_slice(&0x0800u16.to_le_bytes());
            b.extend_from_slice(&8u16.to_le_bytes());
            b.extend_from_slice(&time.to_le_bytes());
            b.extend_from_slice(&date.to_le_bytes());
            b.extend_from_slice(&c.sum().to_le_bytes());
            b.extend_from_slice(&small(packed.len())?.to_le_bytes());
            b.extend_from_slice(&small(data.len())?.to_le_bytes());
            b.extend_from_slice(&name_len.to_le_bytes());
            b.extend_from_slice(&0u16.to_le_bytes());
            Ok(())
        };
        out.extend_from_slice(&LOCAL.to_le_bytes());
        common(&mut out)?;
        out.extend_from_slice(name);
        out.extend_from_slice(&packed);

        central.extend_from_slice(&CENTRAL.to_le_bytes());
        // Made on Unix, so the mode below means a plain file.
        central.extend_from_slice(&((3u16 << 8) | 20).to_le_bytes());
        common(&mut central)?;
        central.extend_from_slice(&0u16.to_le_bytes()); // comment
        central.extend_from_slice(&0u16.to_le_bytes()); // disk
        central.extend_from_slice(&0u16.to_le_bytes()); // internal attributes
        central.extend_from_slice(&(0o100_644u32 << 16).to_le_bytes());
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(name);
    }
    let count = u16::try_from(sorted.len()).map_err(|_| too_large())?;
    let central_at = small(out.len())?;
    let central_len = small(central.len())?;
    out.extend_from_slice(&central);
    out.extend_from_slice(&END.to_le_bytes());
    out.extend_from_slice(&[0; 4]); // this disk, the central directory's disk
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&central_len.to_le_bytes());
    out.extend_from_slice(&central_at.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    Ok(out)
}

/// Whether `path` names a package rather than a folder.
pub fn is_package(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case(EXTENSION))
}

/// Unpacks a package into a new folder under `staging`, named after the plugin's id so it
/// reads like any plugin folder. Returns that folder and the content hash.
pub(super) fn unpack(package: &Path, staging: &Path) -> Result<(PathBuf, String), String> {
    let size = std::fs::metadata(package)
        .map_err(|_| format!("Can't open {}.", package.display()))?
        .len();
    if size > MAX_BYTES {
        return Err(too_large());
    }
    let zip = std::fs::read(package).map_err(|e| format!("Can't read the package: {e}"))?;
    let files = read_zip(&zip)?;
    let manifest = files
        .iter()
        .find(|(p, _)| p == MANIFEST)
        .ok_or_else(|| format!("The package has no {MANIFEST}."))?;
    let text = std::str::from_utf8(&manifest.1).map_err(|_| format!("{MANIFEST} isn't UTF-8."))?;
    let id = bonsai_plugin::Manifest::parse(text)
        .map_err(|e| e.to_string())?
        .id;
    let hash = content_hash(&files);
    let dir = staging.join(&hash[..16]).join(&id);
    let _ = std::fs::remove_dir_all(&dir);
    let err = |e: std::io::Error| format!("Can't unpack the plugin: {e}");
    for (rel, data) in &files {
        let dest = dir.join(rel);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(err)?;
        }
        std::fs::write(dest, data).map_err(err)?;
    }
    Ok((dir, hash))
}

/// Packs the plugin in `dir` into `dest`. Returns the content hash.
pub(super) fn pack(dir: &Path, dest: &Path) -> Result<String, String> {
    let files = read_folder(dir)?;
    if !files.iter().any(|(p, _)| p == MANIFEST) {
        return Err(format!("No {MANIFEST} in {}.", dir.display()));
    }
    let zip = write_zip(&files)?;
    let tmp = dest.with_extension(format!("{EXTENSION}.tmp"));
    std::fs::write(&tmp, zip).map_err(|e| format!("Can't write the package: {e}"))?;
    std::fs::rename(&tmp, dest).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("Can't write the package: {e}")
    })?;
    Ok(content_hash(&files))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(list: &[(&str, &str)]) -> Files {
        list.iter()
            .map(|(p, d)| ((*p).to_owned(), d.as_bytes().to_vec()))
            .collect()
    }

    /// A zip with one entry, written by hand, for entries `write_zip` never makes.
    fn one_entry(name: &str, method: u16, flags: u16, made_by: u16, attrs: u32, data: &[u8]) -> Vec<u8> {
        let mut c = flate2::Crc::new();
        c.update(data);
        let mut out = Vec::new();
        let header = |b: &mut Vec<u8>| {
            b.extend_from_slice(&20u16.to_le_bytes());
            b.extend_from_slice(&flags.to_le_bytes());
            b.extend_from_slice(&method.to_le_bytes());
            b.extend_from_slice(&[0; 4]);
            b.extend_from_slice(&c.sum().to_le_bytes());
            b.extend_from_slice(&(data.len() as u32).to_le_bytes());
            b.extend_from_slice(&(data.len() as u32).to_le_bytes());
            b.extend_from_slice(&(name.len() as u16).to_le_bytes());
            b.extend_from_slice(&0u16.to_le_bytes());
        };
        out.extend_from_slice(&LOCAL.to_le_bytes());
        header(&mut out);
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
        let at = out.len() as u32;
        out.extend_from_slice(&CENTRAL.to_le_bytes());
        out.extend_from_slice(&made_by.to_le_bytes());
        header(&mut out);
        out.extend_from_slice(&[0; 6]);
        out.extend_from_slice(&attrs.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        let len = out.len() as u32 - at;
        out.extend_from_slice(&END.to_le_bytes());
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&at.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    #[test]
    fn packs_and_reads_back_the_same_files() {
        let f = files(&[
            ("plugin.toml", "id = \"x\"\nentry = \"main.luau\""),
            ("main.luau", "return 1"),
            ("lib/util.luau", &"x".repeat(5000)),
        ]);
        let zip = write_zip(&f).unwrap();
        assert!(zip.len() < 1000, "deflated: {}", zip.len());
        let mut back = read_zip(&zip).unwrap();
        back.sort();
        let mut want = f.clone();
        want.sort();
        assert_eq!(back, want);
        assert_eq!(content_hash(&back), content_hash(&f));
        // Same files, same bytes.
        let mut reversed = f.clone();
        reversed.reverse();
        assert_eq!(write_zip(&reversed).unwrap(), zip);
    }

    #[test]
    fn hash_covers_paths_and_contents() {
        let a = content_hash(&files(&[("a", "1"), ("b", "2")]));
        assert_eq!(a, content_hash(&files(&[("b", "2"), ("a", "1")])));
        assert_ne!(a, content_hash(&files(&[("a", "1"), ("b", "3")])));
        assert_ne!(a, content_hash(&files(&[("a", "1"), ("c", "2")])));
        // The length keeps "ab" + "" apart from "a" + "b".
        assert_ne!(
            content_hash(&files(&[("a", "b"), ("c", "")])),
            content_hash(&files(&[("a", ""), ("c", "b")]))
        );
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn drops_one_top_folder() {
        let zip = write_zip(&files(&[
            ("x/plugin.toml", "id = \"x\"\nentry = \"main.luau\""),
            ("x/main.luau", ""),
        ]))
        .unwrap();
        let mut names: Vec<String> = read_zip(&zip).unwrap().into_iter().map(|(p, _)| p).collect();
        names.sort();
        assert_eq!(names, vec!["main.luau", "plugin.toml"]);
    }

    #[test]
    fn refuses_escapes_and_skips_dot_files() {
        for bad in ["../x.luau", "/etc/passwd", "a/../../x", "a\\b", "C:/x", "a//b"] {
            let zip = one_entry(bad, 0, 0, 20, 0, b"x");
            assert!(read_zip(&zip).is_err(), "{bad}");
        }
        let zip = one_entry(".git/HEAD", 0, 0, 20, 0, b"x");
        assert_eq!(read_zip(&zip).unwrap(), Vec::new());
        let link = one_entry("passwd", 0, 0, 3 << 8, 0o120_777 << 16, b"/etc/passwd");
        assert!(read_zip(&link).unwrap_err().contains("symlink"));
        let locked = one_entry("main.luau", 0, 1, 20, 0, b"x");
        assert!(read_zip(&locked).unwrap_err().contains("encrypted"));
        let odd = one_entry("main.luau", 14, 0, 20, 0, b"x");
        assert!(read_zip(&odd).is_err());
        assert!(read_zip(b"not a zip").is_err());
        let mut damaged = one_entry("main.luau", 0, 0, 20, 0, b"hello");
        let i = damaged.windows(5).position(|w| w == b"hello").unwrap();
        damaged[i] = b'j';
        assert!(read_zip(&damaged).is_err());
    }

    #[test]
    fn stops_at_the_size_cap() {
        // Deflates to a few KB, inflates past the cap.
        let mut enc = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
        let chunk = vec![0u8; 1 << 20];
        for _ in 0..=(MAX_BYTES >> 20) {
            enc.write_all(&chunk).unwrap();
        }
        let packed = enc.finish().unwrap();
        let mut zip = one_entry("big.luau", 8, 0, 20, 0, &packed);
        // Claim a small size in the central header; inflating must still stop at the cap.
        let central = zip.windows(4).rposition(|w| w == CENTRAL.to_le_bytes()).unwrap();
        zip[central + 24..central + 28].copy_from_slice(&10u32.to_le_bytes());
        assert!(read_zip(&zip).is_err());
    }

    #[test]
    fn unpacks_into_a_folder_named_after_the_id() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("hello");
        std::fs::create_dir_all(src.join("lib")).unwrap();
        std::fs::write(src.join("plugin.toml"), "id = \"hello\"\nentry = \"main.luau\"").unwrap();
        std::fs::write(src.join("main.luau"), "return 1").unwrap();
        std::fs::write(src.join("lib/a.luau"), "return 2").unwrap();
        std::fs::write(src.join(".secret"), "x").unwrap();
        let pkg = tmp.path().join("hello.bonsaiplugin");
        let hash = pack(&src, &pkg).unwrap();
        assert!(is_package(&pkg));
        assert_eq!(hash, content_hash(&read_folder(&src).unwrap()));
        let (dir, unpacked_hash) = unpack(&pkg, &tmp.path().join(".staging")).unwrap();
        assert_eq!(unpacked_hash, hash);
        assert_eq!(dir.file_name().unwrap(), "hello");
        assert_eq!(
            std::fs::read_to_string(dir.join("lib/a.luau")).unwrap(),
            "return 2"
        );
        assert!(!dir.join(".secret").exists());
    }
}
