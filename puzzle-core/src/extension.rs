use std::path::PathBuf;

pub trait OptionExt {
    fn some(self) -> Option<Self>
    where
        Self: Sized;

    fn some_ref(&self) -> Option<&Self>
    where
        Self: Sized;
}

impl<T> OptionExt for T {
    fn some(self) -> Option<Self> {
        Some(self)
    }

    fn some_ref(&self) -> Option<&Self> {
        Some(self)
    }
}

pub trait PathBufExt {
    fn file_name_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn file_name_string(&self) -> String {
        self.file_name().unwrap().to_string_lossy().into_owned()
    }
}
