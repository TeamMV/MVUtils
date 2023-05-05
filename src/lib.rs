extern crate alloc;

pub mod utils;
pub mod version;
pub mod logger;
pub mod args;
pub mod screen;
pub mod serialize;

#[cfg(test)]
mod tests {
    use crate::utils::next_id;

    #[test]
    fn it_works() {

    }
}
