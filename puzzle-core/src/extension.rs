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
    #[inline(always)]
    fn some(self) -> Option<Self> {
        Some(self)
    }

    #[inline(always)]
    fn some_ref(&self) -> Option<&Self> {
        Some(self)
    }
}

pub trait PathBufExt {
    fn file_name_string(&self) -> String;

    fn canonicalize_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn file_name_string(&self) -> String {
        self.file_name().unwrap().to_string_lossy().into_owned()
    }

    fn canonicalize_string(&self) -> String {
        self.canonicalize().unwrap().to_string_lossy().into_owned()
    }
}

pub trait StringExt {
    fn split_to_vec(&self, pat: &str) -> Vec<String>;
}

impl StringExt for String {
    fn split_to_vec(&self, pat: &str) -> Vec<String> {
        self.split(pat).map(String::from).collect()
    }
}
