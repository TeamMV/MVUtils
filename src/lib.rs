pub mod utils;
pub mod version;
pub mod args;
pub mod screen;
pub mod save;
pub mod unsafe_utils;
pub mod print;
pub mod once;

#[cfg(test)]
mod tests {

    use crate::lazy;
    use crate::once::*;

    #[test]
    fn it_works() {
        lazy! {
            let mut hello: String = "hello".to_string();
        };

        hello.push_str(" world!");

        println!("{}", *hello);
    }
}
