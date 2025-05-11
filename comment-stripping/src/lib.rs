// A simple comment stripping "obfuscator" for C-like source code.

/// Obfuscator that removes comments from source code.
pub struct Obfuscator;

impl Obfuscator {
    /// Creates a new Obfuscator instance.
    pub fn new() -> Self {
        Obfuscator
    }

    /// Obfuscates the input source code by removing comments.
    /// Supports single-line (`//`) and multi-line (`/* */`) comments.
    pub fn obfuscate(&self, input: &str) -> String {
        let mut output = String::new();
        let mut state = State::Code;
        let mut chars = input.chars().peekable();
        let mut prev_char = None;

        while let Some(ch) = chars.next() {
            match state {
                State::Code => match ch {
                    '/' => {
                        if let Some(&next_ch) = chars.peek() {
                            if next_ch == '/' {
                                chars.next(); // Consume '/'
                                state = State::SingleLineComment;
                            } else if next_ch == '*' {
                                chars.next(); // Consume '*'
                                state = State::MultiLineComment;
                            } else {
                                output.push(ch);
                            }
                        } else {
                            output.push(ch);
                        }
                    }
                    '"' | '\'' => {
                        output.push(ch);
                        state = State::String(ch);
                    }
                    _ => output.push(ch),
                },
                State::SingleLineComment => {
                    if ch == '\n' {
                        output.push(ch);
                        state = State::Code;
                    }
                    // Ignore characters until newline
                }
                State::MultiLineComment => {
                    if ch == '*' {
                        if let Some(&next_ch) = chars.peek() {
                            if next_ch == '/' {
                                chars.next(); // Consume '/'
                                state = State::Code;
                            }
                        }
                    }
                    // Ignore characters until '*/'
                }
                State::String(quote) => {
                    output.push(ch);
                    if ch == quote && prev_char != Some('\\') {
                        state = State::Code;
                    }
                }
            }
            prev_char = Some(ch);
        }

        output
    }
}

/// States for the obfuscator's state machine.
#[derive(PartialEq)]
enum State {
    Code,              // Normal code
    SingleLineComment, // Inside // comment
    MultiLineComment,  // Inside /* */ comment
    String(char),      // Inside string literal, with quote type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_comments() {
        let input = "int main() { return 0; }";
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_single_line_comment() {
        let input = "int x = 42; // This is a comment\nint y = 43;";
        let expected = "int x = 42; \nint y = 43;";
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_multi_line_comment() {
        let input = "int x = 42; /* This is a\nmulti-line comment */ int y = 43;";
        let expected = "int x = 42;  int y = 43;";
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_comment_in_string() {
        let input = r#"char* s = "// not a comment";"#;
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, "");
    }

    #[test]
    fn test_only_comments() {
        let input = "// Comment\n/* Another comment */";
        let expected = "\n";
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_string_with_escaped_quotes() {
        let input = r#"char* s = "\"// not a comment\"";"#;
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_mixed_comments_and_code() {
        let input = r#"
        int main() { // Start
            printf("Hello /* world */"); /* Print */
            return 0; // End
        }
        "#;
        let expected = r#"
        int main() { 
            printf("Hello /* world */"); 
            return 0; 
        }
        "#;
        let obfuscator = Obfuscator::new();
        let output = obfuscator.obfuscate(input);
        assert_eq!(output, expected);
    }
}
