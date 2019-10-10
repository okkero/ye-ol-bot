use crate::data::{ParseContext, ParsedCode};

pub struct SortedCodes {
    pub pc: Vec<ParsedCode>,
    pub not_pc: Vec<ParsedCode>,
    pub uncertain: Vec<ParsedCode>,
}

impl SortedCodes {
    fn new() -> Self {
        Self { pc: vec![], not_pc: vec![], uncertain: vec![] }
    }

    pub fn from_unsorted(codes: Vec<ParsedCode>) -> Self {
        let mut sorted_codes = Self::new();
        for code in codes {
            if confident_pc(&code.context) {
                sorted_codes.pc.push(code);
            } else if confident_not_pc(&code.context) {
                sorted_codes.not_pc.push(code);
            } else {
                sorted_codes.uncertain.push(code);
            }
        }

        sorted_codes
    }
}

fn confident_pc(context: &ParseContext) -> bool {
    let prefix_lower = context.line_prefix.to_lowercase();

    prefix_lower.contains("pc")
        || prefix_lower.contains("all platforms")
}

fn confident_not_pc(context: &ParseContext) -> bool {
    let prefix_lower = context.line_prefix.to_lowercase();

    prefix_lower.contains("playstation")
        || prefix_lower.contains("ps4")
        || prefix_lower.contains("ps3")
        || prefix_lower.contains("xbox")
        || prefix_lower.contains("xbone")
        || prefix_lower.contains("xb360")
        || prefix_lower.contains("x360")
}