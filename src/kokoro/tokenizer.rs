use phf::phf_map;
use regex::Regex;

/// Regex string used for cleaning the input.
/// TODO: Lazy or compile time compilation of regex
static CLEANING_REGEX: &str = r#"[^ !\"$',.:;?A-Za-z\u00a1\u00ab\u00bb\u00bf\u00e6\u00e7\u00f0\u00f8\u0127\u014b\u0153\u01c0-\u01c3\u0250-\u0268\u026a-\u0276\u0278-\u027b\u027d\u027e\u0280-\u0284\u0288-\u0292\u0294\u0295\u0298\u0299\u029b-\u029d\u029f\u02a1\u02a2\u02a4\u02a7\u02b0-\u02b2\u02b4\u02b7\u02bc\u02c8\u02cc\u02d0\u02d1\u02de\u02e0\u02e4\u0329\u03b2\u03b8\u03c7\u1d7b\u2014\u201c\u201d\u2026\u2191-\u2193\u2197\u2198\u2c71]"#;

/// Mapping from input characters to token ids of Kokoro.
static VOCAB: phf::Map<char, u32> = phf_map! {
    '$' => 0,
    ';' => 1,
    ':' => 2,
    ',' => 3,
    '.' => 4,
    '!' => 5,
    '?' => 6,
    '\u{00a1}' => 7,
    '\u{00bf}' => 8,
    '\u{2014}' => 9,
    '\u{2026}' => 10,
    '"' => 11,
    '\u{00ab}' => 12,
    '\u{00bb}' => 13,
    '\u{201c}' => 14,
    '\u{201d}' => 15,
    ' ' => 16,
    'A' => 17,
    'B' => 18,
    'C' => 19,
    'D' => 20,
    'E' => 21,
    'F' => 22,
    'G' => 23,
    'H' => 24,
    'I' => 25,
    'J' => 26,
    'K' => 27,
    'L' => 28,
    'M' => 29,
    'N' => 30,
    'O' => 31,
    'P' => 32,
    'Q' => 33,
    'R' => 34,
    'S' => 35,
    'T' => 36,
    'U' => 37,
    'V' => 38,
    'W' => 39,
    'X' => 40,
    'Y' => 41,
    'Z' => 42,
    'a' => 43,
    'b' => 44,
    'c' => 45,
    'd' => 46,
    'e' => 47,
    'f' => 48,
    'g' => 49,
    'h' => 50,
    'i' => 51,
    'j' => 52,
    'k' => 53,
    'l' => 54,
    'm' => 55,
    'n' => 56,
    'o' => 57,
    'p' => 58,
    'q' => 59,
    'r' => 60,
    's' => 61,
    't' => 62,
    'u' => 63,
    'v' => 64,
    'w' => 65,
    'x' => 66,
    'y' => 67,
    'z' => 68,
    '\u{0251}' => 69,
    '\u{0250}' => 70,
    '\u{0252}' => 71,
    '\u{00e6}' => 72,
    '\u{0253}' => 73,
    '\u{0299}' => 74,
    '\u{03b2}' => 75,
    '\u{0254}' => 76,
    '\u{0255}' => 77,
    '\u{00e7}' => 78,
    '\u{0257}' => 79,
    '\u{0256}' => 80,
    '\u{00f0}' => 81,
    '\u{02a4}' => 82,
    '\u{0259}' => 83,
    '\u{0258}' => 84,
    '\u{025a}' => 85,
    '\u{025b}' => 86,
    '\u{025c}' => 87,
    '\u{025d}' => 88,
    '\u{025e}' => 89,
    '\u{025f}' => 90,
    '\u{0284}' => 91,
    '\u{0261}' => 92,
    '\u{0260}' => 93,
    '\u{0262}' => 94,
    '\u{029b}' => 95,
    '\u{0266}' => 96,
    '\u{0267}' => 97,
    '\u{0127}' => 98,
    '\u{0265}' => 99,
    '\u{029c}' => 100,
    '\u{0268}' => 101,
    '\u{026a}' => 102,
    '\u{029d}' => 103,
    '\u{026d}' => 104,
    '\u{026c}' => 105,
    '\u{026b}' => 106,
    '\u{026e}' => 107,
    '\u{029f}' => 108,
    '\u{0271}' => 109,
    '\u{026f}' => 110,
    '\u{0270}' => 111,
    '\u{014b}' => 112,
    '\u{0273}' => 113,
    '\u{0272}' => 114,
    '\u{0274}' => 115,
    '\u{00f8}' => 116,
    '\u{0275}' => 117,
    '\u{0278}' => 118,
    '\u{03b8}' => 119,
    '\u{0153}' => 120,
    '\u{0276}' => 121,
    '\u{0298}' => 122,
    '\u{0279}' => 123,
    '\u{027a}' => 124,
    '\u{027e}' => 125,
    '\u{027b}' => 126,
    '\u{0280}' => 127,
    '\u{0281}' => 128,
    '\u{027d}' => 129,
    '\u{0282}' => 130,
    '\u{0283}' => 131,
    '\u{0288}' => 132,
    '\u{02a7}' => 133,
    '\u{0289}' => 134,
    '\u{028a}' => 135,
    '\u{028b}' => 136,
    '\u{2c71}' => 137,
    '\u{028c}' => 138,
    '\u{0263}' => 139,
    '\u{0264}' => 140,
    '\u{028d}' => 141,
    '\u{03c7}' => 142,
    '\u{028e}' => 143,
    '\u{028f}' => 144,
    '\u{0291}' => 145,
    '\u{0290}' => 146,
    '\u{0292}' => 147,
    '\u{0294}' => 148,
    '\u{02a1}' => 149,
    '\u{0295}' => 150,
    '\u{02a2}' => 151,
    '\u{01c0}' => 152,
    '\u{01c1}' => 153,
    '\u{01c2}' => 154,
    '\u{01c3}' => 155,
    '\u{02c8}' => 156,
    '\u{02cc}' => 157,
    '\u{02d0}' => 158,
    '\u{02d1}' => 159,
    '\u{02bc}' => 160,
    '\u{02b4}' => 161,
    '\u{02b0}' => 162,
    '\u{02b1}' => 163,
    '\u{02b2}' => 164,
    '\u{02b7}' => 165,
    '\u{02e0}' => 166,
    '\u{02e4}' => 167,
    '\u{02de}' => 168,
    '\u{2193}' => 169,
    '\u{2191}' => 170,
    '\u{2192}' => 171,
    '\u{2197}' => 172,
    '\u{2198}' => 173,
    '\u{0329}' => 175,
    '\'' => 176,
    '\u{1d7b}' => 177

};

pub struct KokoroTokenizer;
impl KokoroTokenizer {
    /// Reimplementation of the Kokoro tokenizer, as described in
    /// [onnx-community/Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/blob/main/tokenizer.json).
    /// Step:
    ///     1. Clean input by removing all matches to a regex
    ///     2. Split into single characters
    ///     3. Map characters to token ids
    ///     4. Insert a single padding token (0) at start and end
    ///
    /// # Panics
    /// This function can panic for two reasons:
    ///     - Regex compilation can fail, which should not happen, as it has been tested in practice.
    ///     - Mapping a character to a token id fails. This should not happen, as the regex in the first step
    ///       should in theory remove all invalid chars, so that only those contained in the vocabulary remain.
    pub fn tokenize(text: &str) -> Vec<u32> {
        let re = Regex::new(CLEANING_REGEX)
            .expect("Regex for cleaning Kokoro tokenizer input is not valid.");

        let mut unpadded_result: Vec<u32> = re
            .replace_all(text, "")
            .to_string()
            .chars()
            .map(|c| {
                *VOCAB
                    .get(&c)
                    .expect("Tokenization for Kokoro failed, char >>{c}<< not found in vocabulary.")
            })
            .collect();

        unpadded_result.insert(0, 0);
        unpadded_result.push(0);

        unpadded_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tokenization_test() {
        let test_input = "Hello";
        // Validated result with JS reimplementation of Kokoro tokenizer
        let expected: Vec<u32> = vec![0, 24, 47, 54, 54, 57, 0];

        let result = KokoroTokenizer::tokenize(test_input);
        assert_eq!(result, expected);
    }

    #[test]
    fn complex_tokenization_test() {
        let test_input = "hˌaʊ kʊd aɪ nˈoʊ? ɪts ɐn ʌnˈænsɚɹəbəl kwˈɛstʃən. lˈaɪk ˈæskɪŋ ɐn ʌnbˈɔːɹn tʃˈaɪld ɪf ðeɪl lˈiːd ɐ ɡˈʊd lˈaɪf. ðeɪ hˈævənt ˈiːvən bˌɪn bˈɔːɹn.";
        // Validated result with JS reimplementation of Kokoro tokenizer
        let expected: Vec<u32> = vec![
            0, 50, 157, 43, 135, 16, 53, 135, 46, 16, 43, 102, 16, 56, 156, 57, 135, 6, 16, 102,
            62, 61, 16, 70, 56, 16, 138, 56, 156, 72, 56, 61, 85, 123, 83, 44, 83, 54, 16, 53, 65,
            156, 86, 61, 62, 131, 83, 56, 4, 16, 54, 156, 43, 102, 53, 16, 156, 72, 61, 53, 102,
            112, 16, 70, 56, 16, 138, 56, 44, 156, 76, 158, 123, 56, 16, 62, 131, 156, 43, 102, 54,
            46, 16, 102, 48, 16, 81, 47, 102, 54, 16, 54, 156, 51, 158, 46, 16, 70, 16, 92, 156,
            135, 46, 16, 54, 156, 43, 102, 48, 4, 16, 81, 47, 102, 16, 50, 156, 72, 64, 83, 56, 62,
            16, 156, 51, 158, 64, 83, 56, 16, 44, 157, 102, 56, 16, 44, 156, 76, 158, 123, 56, 4,
            0,
        ];

        let result = KokoroTokenizer::tokenize(test_input);
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_input_test() {
        let test_input = "";
        let expected: Vec<u32> = vec![0, 0];

        let result = KokoroTokenizer::tokenize(test_input);
        assert_eq!(result, expected);
    }

    #[test]
    fn invalid_chars_in_input_test() {
        let test_input = "äääääöööö";
        let expected: Vec<u32> = vec![0, 0];

        let result = KokoroTokenizer::tokenize(test_input);
        assert_eq!(result, expected);
    }

}
