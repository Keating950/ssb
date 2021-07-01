use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    default::Default,
    fmt,
    fs,
    path::Path,
};
use xdg::BaseDirectories;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bookmarks {
    #[serde(flatten)]
    entries: HashMap<String, String>,
}

impl fmt::Display for Bookmarks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self
            .entries
            .keys()
            .map(String::len)
            .max()
            .unwrap_or_default();
        for (i, (k, v)) in self.entries.iter().enumerate() {
            if i < self.entries.len() - 1 {
                writeln!(f, "{:>width$} -> {}", k, v, width = width)?;
            } else {
                write!(f, "{:>width$} -> {}", k, v, width = width)?;
            }
        }
        Ok(())
    }
}

impl Bookmarks {
    const FILENAME: &'static str = "bookmarks.json";

    pub fn new() -> anyhow::Result<Bookmarks> {
        match Bookmarks::base_dirs().find_data_file(Bookmarks::FILENAME) {
            Some(f) => {
                let text = fs::read_to_string(f)?;
                let b: Bookmarks = serde_json::from_str(&text)?;
                Ok(b)
            }
            None => Ok(Bookmarks::default()),
        }
    }

    pub fn insert<T: Into<String>>(&mut self, key: T, val: T) {
        self.entries.insert(key.into(), val.into());
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let save_path = match Bookmarks::base_dirs().find_data_file(Bookmarks::FILENAME) {
            Some(p) => p,
            None => Bookmarks::base_dirs().place_data_file(Bookmarks::FILENAME)?,
        };
        self.save_to_path(save_path)
    }

    fn base_dirs() -> &'static BaseDirectories {
        static XDG_ENTRIES: OnceCell<BaseDirectories> = OnceCell::new();
        XDG_ENTRIES.get_or_init(|| BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).unwrap())
    }

    fn save_to_path<T: AsRef<Path>>(&self, path: T) -> anyhow::Result<()> {
        fs::write(path, serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    macro_rules! assert_ok {
        ($v:ident) => {
            assert!($v.is_ok(), "{}", $v.unwrap_err())
        };
        ($e:expr) => {{
            let tmp = $e;
            assert!(tmp.is_ok(), "{}", tmp.unwrap_err())
        }};
    }

    #[test]
    fn test_serialize() {
        let mut b = Bookmarks::new().unwrap();
        b.insert("hello", "world");
        b.insert("a", "b");
        b.insert("c", "d");
        let json = serde_json::to_string(&b);
        assert_ok!(json);
        println!("{}", json.unwrap());
    }

    #[test]
    fn test_deserialize() {
        let text = read_to_string("test_data/test_data.json").unwrap();
        let b: Result<Bookmarks, _> = serde_json::from_str(&text);
        assert_ok!(b)
    }

    #[test]
    fn test_save_to_path() {
        let text = read_to_string("test_data/test_data.json").unwrap();
        let b: Result<Bookmarks, _> = serde_json::from_str(&text);
        assert_ok!(b.unwrap().save_to_path("test_data/test_save.json"))
    }
}
