#[derive(Debug)]
pub struct ParseContext {
    pub line: String,
    pub line_prefix: String
}

#[derive(Debug)]
pub struct ParsedCode {
    pub code: String,
    pub context: ParseContext,
}