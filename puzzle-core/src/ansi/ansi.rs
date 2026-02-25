use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
pub enum Ansi {
    /* ---------- 重置 ---------- */
    Reset,

    /* ---------- 文本属性 ---------- */
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
    NormalIntensity,
    ItalicOff,
    UnderlineOff,
    BlinkOff,
    ReverseOff,
    HiddenOff,
    StrikethroughOff,

    /* ---------- 文本色 ---------- */
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,

    /* ---------- 背景色 ---------- */
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgBrightBlack,
    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
}

impl Display for Ansi {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let str = match self {
            /* 重置 */
            Ansi::Reset => "\x1B[0m",

            /* 文本属性 */
            Ansi::Bold => "\x1B[1m",
            Ansi::Dim => "\x1B[2m",
            Ansi::Italic => "\x1B[3m",
            Ansi::Underline => "\x1B[4m",
            Ansi::Blink => "\x1B[5m",
            Ansi::Reverse => "\x1B[7m",
            Ansi::Hidden => "\x1B[8m",
            Ansi::Strikethrough => "\x1B[9m",
            Ansi::NormalIntensity => "\x1B[22m",
            Ansi::ItalicOff => "\x1B[23m",
            Ansi::UnderlineOff => "\x1B[24m",
            Ansi::BlinkOff => "\x1B[25m",
            Ansi::ReverseOff => "\x1B[27m",
            Ansi::HiddenOff => "\x1B[28m",
            Ansi::StrikethroughOff => "\x1B[29m",

            /* 前景色 */
            Ansi::Black => "\x1B[30m",
            Ansi::Red => "\x1B[31m",
            Ansi::Green => "\x1B[32m",
            Ansi::Yellow => "\x1B[33m",
            Ansi::Blue => "\x1B[34m",
            Ansi::Magenta => "\x1B[35m",
            Ansi::Cyan => "\x1B[36m",
            Ansi::White => "\x1B[37m",
            Ansi::BrightBlack => "\x1B[90m",
            Ansi::BrightRed => "\x1B[91m",
            Ansi::BrightGreen => "\x1B[92m",
            Ansi::BrightYellow => "\x1B[93m",
            Ansi::BrightBlue => "\x1B[94m",
            Ansi::BrightMagenta => "\x1B[95m",
            Ansi::BrightCyan => "\x1B[96m",
            Ansi::BrightWhite => "\x1B[97m",

            /* 背景色 */
            Ansi::BgBlack => "\x1B[40m",
            Ansi::BgRed => "\x1B[41m",
            Ansi::BgGreen => "\x1B[42m",
            Ansi::BgYellow => "\x1B[43m",
            Ansi::BgBlue => "\x1B[44m",
            Ansi::BgMagenta => "\x1B[45m",
            Ansi::BgCyan => "\x1B[46m",
            Ansi::BgWhite => "\x1B[47m",
            Ansi::BgBrightBlack => "\x1B[100m",
            Ansi::BgBrightRed => "\x1B[101m",
            Ansi::BgBrightGreen => "\x1B[102m",
            Ansi::BgBrightYellow => "\x1B[103m",
            Ansi::BgBrightBlue => "\x1B[104m",
            Ansi::BgBrightMagenta => "\x1B[105m",
            Ansi::BgBrightCyan => "\x1B[106m",
            Ansi::BgBrightWhite => "\x1B[107m",
        };
        f.write_str(str)
    }
}
