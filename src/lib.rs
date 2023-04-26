extern crate alloc;

pub mod utils;
pub mod version;
pub mod logger;
pub mod args;
pub mod screen;
pub mod serialize;

#[cfg(test)]
mod tests {
    use crate::try_catch;
    use crate::utils::SplitInto;

    #[test]
    fn it_works() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let split = vec.split_into(3);
        println!("{:?}", split);

        let value = try_catch!({
            String::from_utf8(vec![b'a', b'b', b'c'])
        }, |e| {
            println!("{}", e);
            return Some(());
        });
        println!("{}", value.unwrap());
    }
}
