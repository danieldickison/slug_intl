#![doc = include_str!("../README.md")]

use finl_unicode::categories::{CharacterCategories, MajorCategory};
use unicode_normalization::UnicodeNormalization;

/// Converts `str` to a `String` suitable for use as a URL path component.
///
/// It first normalizes Unicode as NFC, then makes some aesthetic conversions:
/// * letters are lowercased where applicable
/// * whitespace and punctuation are replaced with hyphen
/// * repeated hyphens are collapsed
/// * leading and trailing hyphens are removed
///
/// Unlike other similar libraries, this maintains non-ASCII Unicode characters, which are well-
/// supported by current browsers using percent-encoding. Percent-encoding is left as an exercise
/// for the caller (because you likely want to store the raw UTF-8 in your database, say).
///
/// ## Examples
///
/// ASCII-only input deals mainly with capitalization, punctuation, and whitespace:
/// ```rust
/// # use slug_intl::slugify;
/// assert_eq!("hello", slugify("Hello"));
/// assert_eq!("hello-world", slugify("Hello World"));
/// assert_eq!("hello-world", slugify("Hello World!"));
/// assert_eq!("hello-world", slugify("/?&#Hello\n\r\n\r   --World!!!"));
/// ```
///
/// Printable Unicode characters are normalized but otherwise preserved:
/// ```rust
/// # use slug_intl::slugify;
/// assert_eq!("おはよう-世界", slugify("おはよう、世界！！"));
/// assert_eq!("おはよう🐠", slugify("おはよう🐠"));
/// /// Hyphen replacement is based on what Unicode considers "punctuation":
/// assert_eq!("1≈3∞5=-£9", slugify("¡¡1≈3∞5=¶£9!!"));
/// /// Unicode is normalized as NFC
/// assert_eq!("am\u{00e9}lie", slugify("ame\u{0301}lie"));
/// ```
///
/// You may want to percent-escape the output when rendering HTML, e.g. with the
/// [urlencoding](https://crates.io/crates/urlencoding) crate:
/// ```rust
/// # use slug_intl::slugify;
/// assert_eq!("hello-%F0%9F%90%A0", urlencoding::encode(&slugify("Hello 🐠")));
/// ```
///
pub fn slugify(str: &str) -> String {
    let mut prev_hyphen = true; // removes leading hyphens by starting true

    let mut process_char = |c: char| match c.get_major_category() {
        MajorCategory::L => {
            prev_hyphen = false;
            c.to_lowercase().to_string()
        }
        MajorCategory::M | MajorCategory::N | MajorCategory::S => {
            prev_hyphen = false;
            c.to_string()
        }
        MajorCategory::P | MajorCategory::Z | MajorCategory::C => {
            if prev_hyphen {
                "".to_string()
            } else {
                prev_hyphen = true;
                "-".to_string()
            }
        }
    };

    // TODO: can we make this more efficient with less copying?
    str.nfc()
        .flat_map(|c| process_char(c).chars().collect::<Vec<_>>())
        .collect::<String>()
        .trim_end_matches("-")
        .to_string()
}
