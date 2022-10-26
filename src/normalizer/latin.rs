use std::borrow::Cow;

use deunicode::deunicode;

use super::Normalizer;
use crate::detection::Script;
use crate::Token;

/// Latin specialized [`Normalizer`] converting unicode chars into Ascii.
///
/// This Normalizer uses [`deunicode`] internally to normalize the provided token.
pub struct LatinNormalizer;

impl Normalizer for LatinNormalizer {
    fn normalize_str<'o>(&self, src: &'o str) -> Cow<'o, str> {
        Cow::Owned(deunicode(src))
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && !token.lemma().is_ascii()
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::NormalizerOption;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("Léopard…".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("Leopard...".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("leopard...".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(LatinNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
