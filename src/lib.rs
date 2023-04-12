pub mod utils;
pub mod version;
pub mod logger;
pub mod args;

#[cfg(test)]
mod tests {
    use std::env;
    use crate::args::ParseArgs;
    use crate::try_catch;

    #[test]
    fn it_works() {
        let value = try_catch!({
            let myRes: Result<i32, String> = Err("hello".to_string());
            myRes
        }, |e| {
            println!("{}", e);
        });
        println!("{}", value.unwrap_or(-1));
    }
}
