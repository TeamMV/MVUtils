pub mod utils;
pub mod version;
pub mod screen;
pub mod save;
pub mod unsafe_utils;
pub mod print;
pub mod once;
pub mod static_vec;
pub mod arguments;
pub mod cinit;

#[cfg(test)]
mod tests {
    use crate::arguments::ParseArgs;

    #[test]
    fn it_works() {
        let args = "--pos -x 20 -y 10 --flags easy creative".split(" ").map(|s| s.to_string());
        let parsed = args.parse_args()
            .heading_prefix("--".to_string())
            .param_name_prefix("-".to_string())
            .parse();

        println!("{}", parsed.heading("pos".to_string()).unwrap().value_of::<i32>("x".to_string()).unwrap());
        println!("{}", parsed.heading("flags".to_string()).unwrap().exists("creative".to_string()));
    }
}
