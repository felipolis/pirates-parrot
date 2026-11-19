//! Languages a dictation can be transcribed in.

use serde::{Deserialize, Serialize};

/// Declares [`Language`] from a single table so that the variant, the locale
/// code sent to the model and the human-readable name never drift apart.
macro_rules! languages {
    ($($(#[$attr:meta])* $variant:ident => $code:literal, $name:literal;)+) => {
        /// The language spoken during a dictation.
        ///
        /// Only the locales the model transcribes reliably out of the box are
        /// listed, plus automatic detection.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
        pub enum Language {
            $($(#[$attr])* #[serde(rename = $code)] $variant,)+
        }

        impl Language {
            /// Every selectable language, automatic detection first.
            pub const ALL: &[Self] = &[$(Self::$variant),+];

            /// Locale code understood by the model, such as `pt-BR`, or `auto`.
            pub const fn code(self) -> &'static str {
                match self {
                    $(Self::$variant => $code,)+
                }
            }

            /// Parses a locale code as returned by [`Language::code`].
            pub fn from_code(code: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|language| language.code() == code)
            }

            /// Name shown to the user.
            pub const fn display_name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)+
                }
            }
        }
    };
}

languages! {
    #[default]
    Auto => "auto", "Detect automatically";
    ArabicArabia => "ar-AR", "Arabic";
    DutchNetherlands => "nl-NL", "Dutch";
    EnglishUnitedKingdom => "en-GB", "English (United Kingdom)";
    EnglishUnitedStates => "en-US", "English (United States)";
    FrenchCanada => "fr-CA", "French (Canada)";
    FrenchFrance => "fr-FR", "French (France)";
    GermanGermany => "de-DE", "German";
    HindiIndia => "hi-IN", "Hindi";
    ItalianItaly => "it-IT", "Italian";
    JapaneseJapan => "ja-JP", "Japanese";
    KoreanKorea => "ko-KR", "Korean";
    PortugueseBrazil => "pt-BR", "Portuguese (Brazil)";
    PortuguesePortugal => "pt-PT", "Portuguese (Portugal)";
    RussianRussia => "ru-RU", "Russian";
    SpanishSpain => "es-ES", "Spanish (Spain)";
    SpanishUnitedStates => "es-US", "Spanish (Latin America)";
    TurkishTurkey => "tr-TR", "Turkish";
    UkrainianUkraine => "uk-UA", "Ukrainian";
    VietnameseVietnam => "vi-VN", "Vietnamese";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn codes_are_unique() {
        let codes: HashSet<_> = Language::ALL.iter().map(|l| l.code()).collect();
        assert_eq!(codes.len(), Language::ALL.len());
    }

    #[test]
    fn serializes_as_locale_code() {
        for &language in Language::ALL {
            let json = serde_json::to_string(&language).unwrap();
            assert_eq!(json, format!("\"{}\"", language.code()));
            assert_eq!(serde_json::from_str::<Language>(&json).unwrap(), language);
        }
    }

    #[test]
    fn defaults_to_automatic_detection() {
        assert_eq!(Language::default(), Language::Auto);
    }
}
