pub struct SourcePosition {
    pub line: u32,
    pub column: u32,
}

pub enum SourceLocation {
    File { start: u32, end: u32 },
    Builtin,
}
