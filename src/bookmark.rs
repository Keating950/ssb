use serde::{Deserialize, Serialize};
use std::{
    ffi::{CString, NulError},
    fmt,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    pub addr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl Bookmark {
    pub fn into_cmd(self) -> Result<Vec<CString>, NulError> {
        match self.args {
            Some(mut args) => Ok(args
                .drain(..)
                .chain([self.addr])
                .map(|s| CString::new(s.as_bytes()))
                .collect::<Result<Vec<CString>, NulError>>()?),
            None => Ok(vec![CString::new(self.addr.as_bytes())?]),
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
    use crate::assert_ok;

    #[test]
    fn test_serialize() {
        let inputs = [
            Bookmark {
                addr: "user@dev".to_string(),
                args: Some(vec!["-i".to_string(), "~/.ssh/id_rsa".to_string()]),
            },
            Bookmark {
                addr: "user@dev".to_string(),
                args: None,
            },
            Bookmark {
                addr: String::new(),
                args: Some(vec![]),
            },
            Bookmark {
                addr: String::new(),
                args: None,
            },
        ];
        for b in inputs {
            assert_ok!(serde_json::to_string(&b))
        }
    }

    #[test]
    fn test_into_cmd() {
        macro_rules! cstr {
            ($s:literal) => {
                CString::new($s).unwrap()
            };
        }
        let b = Bookmark {
            addr: "user@dev".to_string(),
            args: Some(vec!["-i".to_string(), "~/.ssh/id_rsa".to_string()]),
        };
        let cmd = b.into_cmd();
        assert_ok!(cmd);
        assert_eq!(
            cmd.unwrap(),
            vec![cstr!("-i"), cstr!("~/.ssh/id_rsa"), cstr!("user@dev"),]
        );
    }
}
