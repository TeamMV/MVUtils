use std::collections::HashMap;
use std::env::Args;
use crate::sealable;

sealable!();

pub struct ParsedArgs {
    command: String,
    args: HashMap<String, String>
}

impl ParsedArgs {
    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn arg(&self, key: &str) -> Option<String> {
        self.args.get(key).cloned()
    }

    pub fn multi_key_arg(&self, keys: Vec<&str>) -> Option<String> {
        for key in keys {
            let v = self.args.get(key);
            if v.is_some() {
                return v.cloned();
            }
        }
        None
    }
}

sealed!(
    pub trait ParseArgs {
        fn parse(self) -> ParsedArgs;
    }
);

impl ParseArgs for Args {
    fn parse(self) -> ParsedArgs {
        let mut args = HashMap::new();
        let mut command = String::new();
        let mut key: Option<String> = None;
        for (i, arg) in self.enumerate() {
            if i == 0 {
                continue;
            }
            if arg.starts_with("-") {
                key = Some(arg.replace("-", ""));
            }
            else if key.is_none() && i == 1 {
                command = arg.clone();
            }
            else if key.is_some() {
                args.insert(key.unwrap().clone(), arg.clone());
                key = None;
            }
        }
        ParsedArgs {
            command,
            args
        }
    }
}

seal!(Args);