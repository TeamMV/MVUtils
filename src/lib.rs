pub mod utils;
pub mod version;
pub mod args;
pub mod screen;
pub mod save;
pub mod unsafe_utils;

#[cfg(test)]
mod tests {
    use crate::exec::Resolving;
    use crate::unsafe_utils::NullableRc;

    #[test]
    fn it_works() {
        let myint = Resolving::new(String::from("hello"))
            .is(|t| t.len() == 6)
            .then_return_self()
            .else_return(String::from("world"))
            .resolve();

        let mut str = String::from("hello");
        if str.len() != 6 {
            str = String::from("world");
        }
        println!("{}", myint);
    }
}
