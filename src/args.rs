use std::collections::HashMap;
use std::env::Args;
use std::ops::Index;
use crate::sealable;

sealable!();

pub struct ParsedArgs {
    command: Option<String>,
    params: HashMap<String, String>,
    args: Vec<String>,
}

impl ParsedArgs {
    pub fn command(&self) -> Option<&String> {
        self.command.as_ref()
    }

    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn multi_key_param(&self, keys: Vec<&str>) -> Option<&String> {
        for key in keys {
            if let Some(value) = self.params.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn arg(&self, index: usize) -> Option<&String> {
        self.args.get(index)
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}

impl Index<usize> for ParsedArgs {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        self.args.index(index)
    }
}

impl Index<&str> for ParsedArgs {
    type Output = String;

    fn index(&self, key: &str) -> &Self::Output {
        self.params.index(key)
    }
}

sealed!(
    pub trait ParseArgs {
        fn parse(self) -> ParsedArgs;
    }
);

impl ParseArgs for Args {
    fn parse(self) -> ParsedArgs {
        let mut params = HashMap::new();
        let mut args = Vec::new();
        let mut command = None;
        let mut key: Option<String> = None;
        let mut in_args = false;
        for (i, arg) in self.skip(1).enumerate() {
            if in_args {
                args.push(arg);
            }
            else if arg == "--" {
                in_args = true;
            }
            else if arg.starts_with('-') {
                key = Some(arg.replace('-', ""));
            }
            else if i == 1 {
                command = Some(arg);
            }
            else if key.is_some() {
                params.insert(key.unwrap(), arg);
                key = None;
            }
        }
        ParsedArgs {
            command,
            params,
            args
        }
    }
}

seal!(Args);