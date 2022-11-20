use crate::bookmark::Bookmark;
use anyhow::Context;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default::Default, fmt, fs, path::Path};
use xdg::BaseDirectories;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bookmarks {
    #[serde(flatten)]
    entries: HashMap<String, Bookmark>,
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
                writeln!(f, "{k:>width$} -> {v}")?;
            } else {
                write!(f, "{k:>width$} -> {v}")?;
            }
        }
        Ok(())
    }
}

impl Bookmarks {
    const FILENAME: &'static str = "bookmarks.json";

    pub fn load() -> anyhow::Result<Bookmarks> {
        match Bookmarks::base_dirs().find_data_file(Bookmarks::FILENAME) {
            Some(f) => {
                let text = fs::read_to_string(&f)?;
                if text.is_empty() {
                    Ok(Bookmarks::default())
                } else {
                    Ok(serde_json::from_str(&text)
                        .with_context(|| format!("Failed to deserialize {}", f.display()))?)
                }
            }
            None => Ok(Bookmarks::default()),
        }
    }

    pub fn insert(&mut self, key: &str, addr: &str, args: Option<&str>) {
        let b = Bookmark {
            addr: addr.to_string(),
            args: args.map(|s| s.split_ascii_whitespace().map(String::from).collect()),
        };
        self.entries.insert(key.into(), b);
    }

    pub fn get(&self, key: &str) -> Option<&Bookmark> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Bookmark> {
        self.entries.remove(key)
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
        fs::write(path, serde_json::to_string_pretty(self)?.as_bytes())
            .map_err(std::convert::Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_ok;

    #[test]
    fn test_serialize() {
        let mut b = Bookmarks::load().unwrap();
        b.insert("hello", "world", None);
        b.insert("a", "b", Some("-i ~/.ssh/id_rsa"));
        b.insert("c", "d", None);
        let json = serde_json::to_string(&b);
        assert_ok!(json);
        println!("{}", json.unwrap());
    }

    #[test]
    fn test_deserialize() {
        let text = include_str!("../test_data/test_data.json");
        let b: Result<Bookmarks, _> = serde_json::from_str(&text);
        assert_ok!(b)
    }

    #[test]
    fn test_save_to_path() {
        let text = include_str!("../test_data/test_data.json");
        let b: Result<Bookmarks, _> = serde_json::from_str(&text);
        assert_ok!(b);
        assert_ok!(b.unwrap().save_to_path("test_data/test_save.json"))
    }
}
