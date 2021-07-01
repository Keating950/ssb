use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    addr: String,
    args: Option<Vec<String>>,
}

impl Bookmark {
    pub fn as_cmd(mut self) -> Vec<String> {
        match self.args {
            Some(mut args) => {
                args.push(self.addr);
                args
            }
            None => vec![self.addr],
        }
    }
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.args {
            Some(args) => write!(f, "(addr: {}, args: {:?})", self.addr, args),
            None => write!(f, "(addr: {})", self.addr),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_serialize() {
        let inputs = [
            Bookmark {
                addr: "user@dev".to_string(),
                args: Some(vec!["-i".to_string(), "~/.ssh/id_rsa".to_string()]),
            },
            Bookmark {
                addr: String::new(),
                args: Some(vec![]),
            },
        ];
        for b in inputs {
            let res = serde_json::to_string(&b);
            assert!(res.is_ok(), "{}", res.unwrap_err());
        }
    }

    #[test]
    fn test_as_cmd() {
        let b = Bookmark {
            addr: "user@dev".to_string(),
            args: Some(vec!["-i".to_string(), "~/.ssh/id_rsa".to_string()]),
        };
        assert_eq!(
            b.as_cmd(),
            vec![
                "-i".to_string(),
                "~/.ssh/id_rsa".to_string(),
                "user@dev".to_string()
            ]
        );
    }
}
