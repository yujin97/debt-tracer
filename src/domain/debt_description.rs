use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct DebtDescription(String);

impl AsRef<str> for DebtDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl DebtDescription {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_too_long = s.graphemes(true).count() > 256;

        if is_too_long {
            Err(format!("{} is too long for a description", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_ok;

    #[test]
    fn a_256_grapheme_long_description_is_valid() {
        let description = "ё".repeat(256);
        assert_ok!(DebtDescription::parse(description));
    }
}
