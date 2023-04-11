use std::collections::HashMap;
use std::env::Args;

pub struct ParsedArgs {
    command: String,
    args: HashMap<String, String>
}

impl ParsedArgs {
    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn arg(&self, key: &str) -> String {
        if let Some(value) = self.args.get(key) {
            value.clone()
        }
        else {
            String::new()
        }
    }

    pub fn multi_key_arg(&self, keys: Vec<&str>) -> String {
        for key in keys {
            if let Some(value) = self.args.get(key) {
                return value.clone();
            }
        }
        String::new()
    }
}

pub trait ParseArgs {
    fn parse(self) -> ParsedArgs;
}

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