/// Abbreviated English weekday names, indexed by weekday number.
pub const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Abbreviated English month names, indexed by month number minus one.
pub const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Day count of every month in a leap year.
pub const LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Day count of every month in a common (non-leap) year.
pub const COMMON_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Environment variable holding the locale of the current session.
pub const LANG_ENV_VAR: &str = "LANG";

/// Human-readable name of the Arabic (Saudi Arabia) locale.
pub const LANG_AR_SA_NAME: &str = "العربية (السعودية)";

/// Human-readable name of the German (Germany) locale.
pub const LANG_DE_DE_NAME: &str = "Deutsch (Deutschland)";

/// Human-readable name of the English (United States) locale.
pub const LANG_EN_US_NAME: &str = "English (US)";

/// Human-readable name of the Spanish (Spain) locale.
pub const LANG_ES_ES_NAME: &str = "Español (España)";

/// Human-readable name of the Finnish (Finland) locale.
pub const LANG_FI_FI_NAME: &str = "Suomi (Suomi)";

/// Human-readable name of the French (France) locale.
pub const LANG_FR_FR_NAME: &str = "Français (France)";

/// Human-readable name of the Hindi (India) locale.
pub const LANG_HI_IN_NAME: &str = "हिन्दी (भारत)";

/// Human-readable name of the Italian (Italy) locale.
pub const LANG_IT_IT_NAME: &str = "Italiano (Italia)";

/// Human-readable name of the Japanese (Japan) locale.
pub const LANG_JA_JP_NAME: &str = "日本語 (日本)";

/// Human-readable name of the Korean (South Korea) locale.
pub const LANG_KO_KR_NAME: &str = "한국어 (한국)";

/// Human-readable name of the Dutch (Netherlands) locale.
pub const LANG_NL_NL_NAME: &str = "Nederlands (Nederland)";

/// Human-readable name of the Portuguese (Portugal) locale.
pub const LANG_PT_PT_NAME: &str = "Português (Portugal)";

/// Human-readable name of the Russian (Russia) locale.
pub const LANG_RU_RU_NAME: &str = "Русский (Россия)";

/// Human-readable name of the Swedish (Sweden) locale.
pub const LANG_SV_SE_NAME: &str = "Svenska (Sverige)";

/// Human-readable name of the Thai (Thailand) locale.
pub const LANG_TH_TH_NAME: &str = "ภาษาไทย (ประเทศไทย)";

/// Human-readable name of the Vietnamese (Vietnam) locale.
pub const LANG_VI_VN_NAME: &str = "Tiếng Việt (Việt Nam)";

/// Human-readable name of the Chinese (China) locale.
pub const LANG_ZH_CN_NAME: &str = "中文 (中国)";

/// POSIX locale string of the Arabic (Saudi Arabia) UTF-8 encoding.
pub const LANG_AR_SA_LOCALE: &str = "ar_SA.UTF-8";

/// POSIX locale string of the German (Germany) UTF-8 encoding.
pub const LANG_DE_DE_LOCALE: &str = "de_DE.UTF-8";

/// POSIX locale string of the English (United States) UTF-8 encoding.
pub const LANG_EN_US_LOCALE: &str = "en_US.UTF-8";

/// POSIX locale string of the Spanish (Spain) UTF-8 encoding.
pub const LANG_ES_ES_LOCALE: &str = "es_ES.UTF-8";

/// POSIX locale string of the Finnish (Finland) UTF-8 encoding.
pub const LANG_FI_FI_LOCALE: &str = "fi_FI.UTF-8";

/// POSIX locale string of the French (France) UTF-8 encoding.
pub const LANG_FR_FR_LOCALE: &str = "fr_FR.UTF-8";

/// POSIX locale string of the Hindi (India) UTF-8 encoding.
pub const LANG_HI_IN_LOCALE: &str = "hi_IN.UTF-8";

/// POSIX locale string of the Italian (Italy) UTF-8 encoding.
pub const LANG_IT_IT_LOCALE: &str = "it_IT.UTF-8";

/// POSIX locale string of the Japanese (Japan) UTF-8 encoding.
pub const LANG_JA_JP_LOCALE: &str = "ja_JP.UTF-8";

/// POSIX locale string of the Korean (South Korea) UTF-8 encoding.
pub const LANG_KO_KR_LOCALE: &str = "ko_KR.UTF-8";

/// POSIX locale string of the Dutch (Netherlands) UTF-8 encoding.
pub const LANG_NL_NL_LOCALE: &str = "nl_NL.UTF-8";

/// POSIX locale string of the Portuguese (Portugal) UTF-8 encoding.
pub const LANG_PT_PT_LOCALE: &str = "pt_PT.UTF-8";

/// POSIX locale string of the Russian (Russia) UTF-8 encoding.
pub const LANG_RU_RU_LOCALE: &str = "ru_RU.UTF-8";

/// POSIX locale string of the Swedish (Sweden) UTF-8 encoding.
pub const LANG_SV_SE_LOCALE: &str = "sv_SE.UTF-8";

/// POSIX locale string of the Thai (Thailand) UTF-8 encoding.
pub const LANG_TH_TH_LOCALE: &str = "th_TH.UTF-8";

/// POSIX locale string of the Vietnamese (Vietnam) UTF-8 encoding.
pub const LANG_VI_VN_LOCALE: &str = "vi_VN.UTF-8";

/// POSIX locale string of the Chinese (China) UTF-8 encoding.
pub const LANG_ZH_CN_LOCALE: &str = "zh_CN.UTF-8";
