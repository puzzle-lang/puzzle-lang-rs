use crate::ansi::ansi::Ansi;

#[macro_export]
macro_rules! ansi_println {
    ($($arg:tt)*) => {{
        let str = format!($($arg)*);
        println!("{}{}", str, Ansi::Reset);
    }};
}

pub trait AddAnsi {
    fn push_ansi(&mut self, ansi: Ansi);
}

impl AddAnsi for String {
    fn push_ansi(&mut self, ansi: Ansi) {
        self.push_str(ansi.to_str());
    }
}
