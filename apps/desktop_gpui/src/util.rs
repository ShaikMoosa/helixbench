//! Small shared helpers for the GPUI shell.

use appcore::state::{AppState, SearchMode};
use domain::Range;

/// Ranges highlighted by the active DNA/protein search (empty for name search).
pub fn search_hit_ranges(app: &AppState) -> Vec<Range> {
    if !app.search.open {
        return Vec::new();
    }
    match app.search.mode {
        SearchMode::Dna => app
            .search
            .dna_hits
            .iter()
            .map(|h| Range::new(h.start, h.len))
            .collect(),
        SearchMode::Protein => app
            .search
            .protein_hits
            .iter()
            .map(|h| Range::new(h.start, h.len))
            .collect(),
        SearchMode::Name => Vec::new(),
    }
}

/// Current unix seconds.
pub fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Unix seconds -> (year, month 1-12, day 1-31), proleptic Gregorian (UTC).
pub fn civil_from_unix(ts: u64) -> (i64, u32, u32) {
    let z = (ts / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// (year, month, day) -> unix seconds at 00:00 UTC.
pub fn unix_from_civil(y: i64, m: u32, d: u32) -> u64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 } as i64;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    (days * 86400) as u64
}

pub fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

pub fn days_in_month(y: i64, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if is_leap(y) {
                29
            } else {
                28
            }
        }
    }
}

/// Weekday of a date, 0 = Sunday .. 6 = Saturday.
pub fn weekday(y: i64, m: u32, d: u32) -> u32 {
    (((unix_from_civil(y, m, d) / 86400) as i64 + 4).rem_euclid(7)) as u32
}

/// Step a (year, month) by `delta` months.
pub fn add_months(y: i64, m: u32, delta: i64) -> (i64, u32) {
    let total = y * 12 + (m as i64 - 1) + delta;
    let ny = total.div_euclid(12);
    let nm = (total.rem_euclid(12) + 1) as u32;
    (ny, nm)
}

#[cfg(test)]
mod timestamp_tests {
    #[test]
    fn checkpoint_time_handles_utc_date_and_leap_day_boundaries() {
        assert_eq!(super::timestamp_utc(0), "1970-01-01 00:00:00 UTC");
        assert_eq!(super::timestamp_utc(1709164799), "2024-02-28 23:59:59 UTC");
        assert_eq!(super::timestamp_utc(1709164800), "2024-02-29 00:00:00 UTC");
        assert_eq!(super::timestamp_utc(u64::MAX), "Timestamp unavailable");
    }
}

/// Display a persisted instant without guessing the reader's local timezone.
pub fn timestamp_utc(ts: u64) -> String {
    if ts > 253402300799 {
        return "Timestamp unavailable".into();
    }
    let (y, m, d) = civil_from_unix(ts);
    let seconds = ts % 86400;
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02} UTC",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}
