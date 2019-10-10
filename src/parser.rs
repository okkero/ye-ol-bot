use crate::data::{ParseContext, ParsedCode};

struct LinePrefix {
    chars: Vec<char>,
    tentative: Vec<char>
}

impl LinePrefix {
    fn new() -> Self {
        Self {
            chars: Vec::with_capacity(20),
            tentative: Vec::with_capacity(20)
        }
    }

    fn bank(&mut self) {
        self.chars.append(&mut self.tentative)
    }

    fn push(&mut self, c: char) {
        self.tentative.push(c);
    }

    fn drop_tentative(&mut self) {
        self.tentative.clear();
    }
}

enum State {
    Find,
    Read,
}

fn find_matches(c: char) -> bool {
    c.is_ascii_alphabetic() && c.is_ascii_uppercase() || c.is_ascii_digit()
}

fn read_matches(c: char, char_count: usize) -> bool {
    match char_count {
        5 | 11 | 17 | 23 => c == '-',
        _ => find_matches(c)
    }
}

pub fn parse_codes(s: &str) -> Vec<ParsedCode> {
    const CODE_LENGTH: usize = 29;

    let mut codes = Vec::with_capacity(3);

    for line in s.lines() {
        let mut state = State::Find;
        let mut code_chars: Vec<char> = Vec::with_capacity(CODE_LENGTH);
        let mut line_prefix = LinePrefix::new();
        let mut found_code = false;

        for c in line.chars() {
            match state {
                State::Find => {
                    if find_matches(c) {
                        line_prefix.bank();
                        code_chars.push(c);
                        state = State::Read;
                    }
                    if !found_code {
                        line_prefix.push(c);
                    }
                }
                State::Read => {
                    if !found_code {
                        line_prefix.push(c);
                    }

                    if !read_matches(c, code_chars.len()) {
                        state = State::Find;
                        code_chars.clear();
                        continue;
                    }

                    code_chars.push(c);
                    if code_chars.len() == 29 {
                        codes.push(ParsedCode {
                            code: code_chars.into_iter().collect(),
                            context: ParseContext {
                                line: line.to_string(),
                                line_prefix: line_prefix.chars.iter().collect()
                            },
                        });
                        found_code = true;
                        line_prefix.drop_tentative();
                        state = State::Find;
                        code_chars = Vec::with_capacity(CODE_LENGTH);
                    }
                }
            }
        }
    }

    codes
}