use finl_unicode::categories::{CharacterCategories, MajorCategory};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct SlugOptions {
    min_len: usize,
    max_len: usize,
}

impl Default for SlugOptions {
    fn default() -> Self {
        Self {
            min_len: 3,
            max_len: 30,
        }
    }
}

/// Converts `str` to a `String` suitable for use as a URL path component. It first normalizes
/// Unicode as NFC, then makes some aesthetic conversions:
/// * letters are lowercased where applicable
/// * whitespace and punctuation are replaced with hyphen
/// * repeated hyphens are collapsed
/// * leading and trailing hyphens are removed
///
/// Unlike other similar libraries, this maintains non-ASCII Unicode characters, which are well-
/// supported by current browsers using percent-encoding. Percent-encoding is left as an exercise
/// for the caller (because you likely want to store the raw UTF-8 in your database, say).
///
/// # Examples
///
/// ```rust
/// # use slug_intl::slugify;
/// assert_eq!(slugify("hello"), "hello".to_string());
/// assert_eq!(slugify("hello world"), "hello-world".to_string());
/// assert_eq!(slugify("hello world!"), "hello-world".to_string());
/// assert_eq!(slugify("hello --world!!!"), "hello-world".to_string());
/// ```
pub fn slugify(str: &str) -> String {
    slugify_with_opts(str, &SlugOptions::default())
}

pub fn slugify_with_opts(str: &str, opts: &SlugOptions) -> String {
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
