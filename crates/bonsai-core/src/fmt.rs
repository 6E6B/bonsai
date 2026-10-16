//! Short number and time formats.

/// 1234 → "1,234"; 1_250_000 → "1.2M".
pub fn count(n: i64) -> String {
    let a = n.unsigned_abs();
    let sign = if n < 0 { "-" } else { "" };
    if a >= 1_000_000_000 {
        format!("{sign}{:.1}B", a as f64 / 1e9)
    } else if a >= 1_000_000 {
        format!("{sign}{:.1}M", a as f64 / 1e6)
    } else if a >= 100_000 {
        format!("{sign}{}K", a / 1000)
    } else {
        let s = a.to_string();
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            if i > 0 && (s.len() - i).is_multiple_of(3) {
                out.push(',');
            }
            out.push(c);
        }
        format!("{sign}{out}")
    }
}

/// Seconds ago → "just now", "5m ago", "3h ago", "2d ago".
pub fn ago(secs: i64) -> String {
    match secs {
        s if s < 45 => "just now".into(),
        s if s < 3600 => format!("{}m ago", (s + 30) / 60),
        s if s < 86_400 => format!("{}h ago", s / 3600),
        s => format!("{}d ago", s / 86_400),
    }
}

/// Cut `s` to at most `max` characters, ending in "…" when cut.
pub fn ellipsize(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.truncate(out.trim_end().len());
    out.push('…');
    out
}

/// Exact amount with thousands separators: 279900 → "279,900". Prices never round.
pub fn exact(n: i64) -> String {
    let s = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 { format!("-{out}") } else { out }
}

/// "R$ 279,900".
pub fn robux(n: i64) -> String {
    format!("R$ {}", exact(n))
}

/// Seconds from now → "in 5m", "in 3h", "in 2d"; past times use [`ago`].
pub fn until(secs: i64) -> String {
    match secs {
        s if s < 0 => ago(-s),
        s if s < 3600 => format!("in {}m", (s + 59) / 60),
        s if s < 86_400 => format!("in {}h", s / 3600),
        s => format!("in {}d", s / 86_400),
    }
}

/// `YYYY-MM-DDTHH:MM:SS[.fff]Z` → unix seconds. Roblox sends 0 to 7 fractional digits.
pub fn parse_iso(s: &str) -> Option<i64> {
    let n = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
    let (y, m, d) = (n(0..4)?, n(5..7)?, n(8..10)?);
    let (hh, mm, ss) = (
        n(11..13).unwrap_or(0),
        n(14..16).unwrap_or(0),
        n(17..19).unwrap_or(0),
    );
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    // Days-from-civil (Howard Hinnant).
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + hh * 3600 + mm * 60 + ss)
}

/// Like/dislike ratio as a percent.
pub fn rating(up: Option<i64>, down: Option<i64>) -> Option<String> {
    let (u, d) = (up?, down?);
    let t = u + d;
    (t > 0).then(|| format!("{}%", (u * 100 + t / 2) / t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ellipsizes() {
        assert_eq!(ellipsize("Adopt Me!", 20), "Adopt Me!");
        assert_eq!(ellipsize("Work at a Pizza Place", 10), "Work at a…");
        assert_eq!(ellipsize("ab cd", 4), "ab…");
    }

    #[test]
    fn counts() {
        assert_eq!(count(0), "0");
        assert_eq!(count(1234), "1,234");
        assert_eq!(count(99_999), "99,999");
        assert_eq!(count(250_000), "250K");
        assert_eq!(count(1_250_000), "1.2M");
        assert_eq!(count(3_400_000_000), "3.4B");
    }

    #[test]
    fn times_and_ratings() {
        assert_eq!(ago(10), "just now");
        assert_eq!(ago(300), "5m ago");
        assert_eq!(ago(7200), "2h ago");
        assert_eq!(rating(Some(90), Some(10)).as_deref(), Some("90%"));
        assert_eq!(rating(Some(0), Some(0)), None);
    }
}

/// Unix seconds as `YYYY-MM-DDTHH:MM:SSZ` (UTC).
pub fn iso_utc(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    // Civil-from-days (Howard Hinnant).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

#[cfg(test)]
mod iso_tests {
    #[test]
    fn iso_dates() {
        assert_eq!(super::iso_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(super::iso_utc(1_791_849_600), "2026-10-13T00:00:00Z");
    }

    #[test]
    fn parses_iso_and_formats_money() {
        use super::*;
        assert_eq!(parse_iso("2026-10-13T00:00:00Z"), Some(1_791_849_600));
        assert_eq!(
            parse_iso("2008-01-26T06:12:24.623Z").map(iso_utc).as_deref(),
            Some("2008-01-26T06:12:24Z")
        );
        assert_eq!(parse_iso("junk"), None);
        assert_eq!(robux(279_900), "R$ 279,900");
        assert_eq!(exact(-1234), "-1,234");
        assert_eq!(until(3 * 86_400 + 5), "in 3d");
        assert_eq!(until(-7200), "2h ago");
    }
}

static LOCAL_OFFSET: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

/// Set the local time zone's offset from UTC in seconds (Core does this at start).
pub fn set_local_offset(secs: i64) {
    LOCAL_OFFSET.store(secs, std::sync::atomic::Ordering::Relaxed);
}

/// Seconds to add to UTC for local wall-clock time.
pub fn local_offset() -> i64 {
    LOCAL_OFFSET.load(std::sync::atomic::Ordering::Relaxed)
}

/// An ISO time as local "HH:MM".
pub fn clock(iso: &str) -> String {
    match parse_iso(iso) {
        Some(t) => {
            let l = (t + local_offset()).rem_euclid(86_400);
            format!("{:02}:{:02}", l / 3600, l % 3600 / 60)
        }
        None => String::new(),
    }
}

/// Local day number (days since 1970-01-01 in local time) of unix seconds.
pub fn local_day(secs: i64) -> i64 {
    (secs + local_offset()).div_euclid(86_400)
}

#[cfg(test)]
mod clock_tests {
    #[test]
    fn local_clock() {
        super::set_local_offset(2 * 3600);
        assert_eq!(super::clock("2026-10-13T23:30:00Z"), "01:30");
        assert_eq!(
            super::local_day(super::parse_iso("2026-10-13T23:30:00Z").unwrap_or(0)),
            20_740
        );
        super::set_local_offset(0);
        assert_eq!(super::clock("junk"), "");
    }
}
