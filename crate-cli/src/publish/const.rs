/// Seconds in a minute, used when converting a parsed timestamp.
pub(crate) const SECONDS_PER_MINUTE: i64 = 60;

/// Seconds in an hour, used when converting a parsed timestamp.
pub(crate) const SECONDS_PER_HOUR: i64 = 60 * SECONDS_PER_MINUTE;

/// Seconds in a day, used when converting a parsed timestamp.
pub(crate) const SECONDS_PER_DAY: i64 = 24 * SECONDS_PER_HOUR;

/// Separator between the fields of an RFC 2822 clock.
pub(crate) const CLOCK_FIELD_SEPARATOR: char = ':';

/// Trailing punctuation on a day-of-month token, as in `Tue, 29 Sep 2026`.
pub(crate) const DAY_FIELD_SUFFIX: char = ',';

/// Divisor rounding the March-based month position into a day of year.
pub(crate) const MONTH_POSITION_DIVISOR: i64 = 5;

/// Rounding term in the March-based day-of-year formula.
pub(crate) const MONTH_POSITION_ROUNDING: i64 = 2;

/// Days in a common year, before leap days are added back.
pub(crate) const DAYS_PER_YEAR: i64 = 365;

/// Years in the leap-year cycle.
pub(crate) const LEAP_CYCLE_YEARS: i64 = 4;

/// Years in a century, which the leap-year cycle skips.
pub(crate) const CENTURY_YEARS: i64 = 100;

/// Years in the 400-year era the civil-date algorithm counts in.
pub(crate) const ERA_YEARS: i64 = 400;

/// Days in the 400-year era, which carries 97 leap days.
pub(crate) const ERA_DAYS: i64 = 146_097;

/// Days between 0000-03-01 and 1970-01-01 in the era-based day count.
pub(crate) const CIVIL_EPOCH_OFFSET_DAYS: i64 = 719_468;

/// Multiplier carrying a month into its March-based position.
pub(crate) const MONTH_POSITION_SCALE: i64 = 153;

/// Fallback wait when a rate limit carries no readable deadline.
///
/// Only used when the registry's message could not be parsed. A parsed
/// deadline is honoured as given — see `RATE_LIMIT_SKEW_SECS` — so this
/// never inflates a window the registry was precise about. Ten minutes is
/// the registry's new-crate refill interval, so an unreadable refusal gets
/// one full interval instead of a guess that would only earn a second one.
pub(crate) const RATE_LIMIT_FLOOR_SECS: u64 = 11 * SECONDS_PER_MINUTE as u64;

/// Added to the wait crates.io asks for, absorbing clock skew between the
/// publishing runner and the registry.
///
/// This is also the shortest wait a refusal can produce. A named deadline
/// is waited out exactly, so a registry that opens the window in thirty
/// seconds costs thirty seconds rather than a guessed interval.
pub(crate) const RATE_LIMIT_SKEW_SECS: u64 = 30;

/// Ceiling on how many rate-limit deadlines a single package may wait out.
///
/// A limit that never lifts must fail the job rather than hold a runner
/// until it is killed. The ceiling only matters while a workspace is
/// publishing several new names in one run: each name costs the registry's
/// refill interval, so this covers a large first release and still stops.
pub(crate) const RATE_LIMIT_MAX_WAITS: u32 = 24;

/// Ceiling on a single wait, in seconds.
///
/// The registry's own refill interval is about ten minutes, so no genuine
/// deadline is anywhere near this bound. It exists because the wait is
/// derived from a parsed timestamp: a mis-parsed or far-future deadline
/// would otherwise sleep the runner for days, and a wrong parse must fail
/// visibly rather than stall the release. Five hours still spans a
/// generous outage, and a workspace that needs longer fails fast enough to
/// be re-run.
pub(crate) const RATE_LIMIT_MAX_WAIT_SECS: u64 = 5 * 60 * 60;

/// Month abbreviations in the RFC 2822 timestamp format crates.io emits.
pub(crate) const MONTH_ABBREVIATIONS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
