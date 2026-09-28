use super::*;

/// Implementation of Display trait for Lang.
///
/// Provides a human-readable string representation for each language variant.
impl fmt::Display for Lang {
    /// Formats the language for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang_str: &str = match self {
            Lang::EnUsUtf8 => LANG_EN_US_NAME,
            Lang::ZhCnUtf8 => LANG_ZH_CN_NAME,
            Lang::FrFrUtf8 => LANG_FR_FR_NAME,
            Lang::DeDeUtf8 => LANG_DE_DE_NAME,
            Lang::EsEsUtf8 => LANG_ES_ES_NAME,
            Lang::ItItUtf8 => LANG_IT_IT_NAME,
            Lang::JaJpUtf8 => LANG_JA_JP_NAME,
            Lang::KoKrUtf8 => LANG_KO_KR_NAME,
            Lang::PtPtUtf8 => LANG_PT_PT_NAME,
            Lang::RuRuUtf8 => LANG_RU_RU_NAME,
            Lang::ArSaUtf8 => LANG_AR_SA_NAME,
            Lang::HiInUtf8 => LANG_HI_IN_NAME,
            Lang::ThThUtf8 => LANG_TH_TH_NAME,
            Lang::ViVnUtf8 => LANG_VI_VN_NAME,
            Lang::NlNlUtf8 => LANG_NL_NL_NAME,
            Lang::SvSeUtf8 => LANG_SV_SE_NAME,
            Lang::FiFiUtf8 => LANG_FI_FI_NAME,
        };
        write!(f, "{lang_str}")
    }
}
impl Lang {
    /// Returns the UTC offset in seconds for the corresponding language.
    ///
    /// Each language is associated with a specific UTC offset,
    /// indicating the difference from Coordinated Universal Time (UTC).
    ///
    /// # Returns
    ///
    /// - `u64` - The UTC offset in seconds.
    pub fn value(&self) -> u64 {
        match self {
            Lang::EnUsUtf8 => 0,     // UTC
            Lang::ZhCnUtf8 => 28800, // UTC+8
            Lang::FrFrUtf8 => 3600,  // UTC+1
            Lang::DeDeUtf8 => 3600,  // UTC+1
            Lang::EsEsUtf8 => 3600,  // UTC+1
            Lang::ItItUtf8 => 3600,  // UTC+1
            Lang::JaJpUtf8 => 32400, // UTC+9
            Lang::KoKrUtf8 => 32400, // UTC+9
            Lang::PtPtUtf8 => 3600,  // UTC+1
            Lang::RuRuUtf8 => 10800, // UTC+3
            Lang::ArSaUtf8 => 10800, // UTC+3
            Lang::HiInUtf8 => 19800, // UTC+5:30
            Lang::ThThUtf8 => 25200, // UTC+7
            Lang::ViVnUtf8 => 25200, // UTC+7
            Lang::NlNlUtf8 => 3600,  // UTC+1
            Lang::SvSeUtf8 => 3600,  // UTC+1
            Lang::FiFiUtf8 => 3600,  // UTC+1
        }
    }
}

/// Implementation of FromStr trait for Lang.
///
/// Allows parsing a string into a Lang variant.
impl FromStr for Lang {
    /// The error type for parsing operations.
    type Err = ();

    /// Parses a string into a Lang variant.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The parsed Lang variant or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            LANG_EN_US_LOCALE => Ok(Lang::EnUsUtf8),
            LANG_ZH_CN_LOCALE => Ok(Lang::ZhCnUtf8),
            LANG_FR_FR_LOCALE => Ok(Lang::FrFrUtf8),
            LANG_DE_DE_LOCALE => Ok(Lang::DeDeUtf8),
            LANG_ES_ES_LOCALE => Ok(Lang::EsEsUtf8),
            LANG_IT_IT_LOCALE => Ok(Lang::ItItUtf8),
            LANG_JA_JP_LOCALE => Ok(Lang::JaJpUtf8),
            LANG_KO_KR_LOCALE => Ok(Lang::KoKrUtf8),
            LANG_PT_PT_LOCALE => Ok(Lang::PtPtUtf8),
            LANG_RU_RU_LOCALE => Ok(Lang::RuRuUtf8),
            LANG_AR_SA_LOCALE => Ok(Lang::ArSaUtf8),
            LANG_HI_IN_LOCALE => Ok(Lang::HiInUtf8),
            LANG_TH_TH_LOCALE => Ok(Lang::ThThUtf8),
            LANG_VI_VN_LOCALE => Ok(Lang::ViVnUtf8),
            LANG_NL_NL_LOCALE => Ok(Lang::NlNlUtf8),
            LANG_SV_SE_LOCALE => Ok(Lang::SvSeUtf8),
            LANG_FI_FI_LOCALE => Ok(Lang::FiFiUtf8),
            _ => Err(()),
        }
    }
}
