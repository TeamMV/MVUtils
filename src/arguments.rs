use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

pub struct ArgSpecs<I> {
    iter: I,

    heading_prefix: String,
    param_name_prefix: String,
}

impl<I: Iterator<Item=String>> ArgSpecs<I> {
    pub fn heading_prefix(mut self, prefix: String) -> Self {
        self.heading_prefix = prefix;
        self
    }

    pub fn param_name_prefix(mut self, prefix: String) -> Self {
        self.param_name_prefix = prefix;
        self
    }

    pub fn parse(mut self) -> ParsedArgs {
        let mut result = ParsedArgs { headings: vec![] };
        let mut in_heading = false;
        let mut in_param = false;
        let mut current_param: (String, String) = ("".to_string(), "".to_string());
        let mut heading = None;
        while let Some(arg) = self.iter.next() {
            if arg.starts_with(&self.heading_prefix) && in_heading {
                in_heading = false;
            }
            if arg.starts_with(&self.heading_prefix) || in_heading {
                if !in_heading && !in_param {
                    heading = Some(Heading::new(arg[self.heading_prefix.len()..].to_string()));
                    result.headings.push(heading.unwrap());
                    in_heading = true;
                } else {
                    if !in_param {
                        if arg.starts_with(&self.param_name_prefix) {
                            in_param = true;
                            current_param.0 = arg[self.param_name_prefix.len()..].to_string();
                        }
                    } else {
                        if arg.starts_with(&self.param_name_prefix) {
                            let mut heading = heading.unwrap();
                            heading.params.insert(current_param.0.clone(), "".to_string());
                            heading.params.insert(arg[self.param_name_prefix.len()..].to_string(), "".to_string());
                            break;
                        }
                        current_param.1 = arg;
                        let mut heading = heading.unwrap();
                        heading.params.insert(current_param.0.clone(), current_param.1);
                        in_param = false;
                    }
                }
            }
        }
        result
    }
}

pub struct ParsedArgs {
    headings: Vec<Heading>
}

impl ParsedArgs {
    pub fn heading(&self, key: String) -> Option<&'_ Heading> {
        self.headings.iter().filter(|h| h.name.eq(&key)).next()
    }
}

pub struct Heading {
    name: String,
    params: HashMap<String, String>
}

impl Heading {
    fn new(name: String) -> Self {
        Self {
            name,
            params: HashMap::new(),
        }
    }

    pub fn param_count(&self) -> usize {
        self.params.len()
    }

    pub fn value_of<Type: FromStr>(&self, key: String) -> Option<Type> where <Type as FromStr>::Err: Debug {
        if self.params.contains_key(&key) {
            return Some(self.params.get(&key).unwrap().parse::<Type>().unwrap())
        }
        None
    }

    pub fn exists(&self, key: String) -> bool {
        self.params.contains_key(&key)
    }
}

pub trait ParseArgs<I> {
    fn parse_args(self) -> ArgSpecs<I>;
}

impl<I: Iterator<Item=String>> ParseArgs<I> for I {
    fn parse_args(self) -> ArgSpecs<I> {
        ArgSpecs {
            iter: self,
            heading_prefix: "--".to_string(),
            param_name_prefix: "-".to_string(),
        }
    }
}