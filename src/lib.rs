pub mod utils;
pub mod version;
pub mod logger;
pub mod args;

#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::try_catch;

    #[test]
    fn it_works() {
        let value = try_catch!({
            let my_res: Result<i32, String> = Err("hello".to_string());
            my_res
        }, |e| {
            println!("{}", e);
        });

        if value.is_none() {
            return;
        }

        println!("{}", value.unwrap_or(-1));
    }
}
