pub mod args;
pub mod cinit;
pub mod once;
pub mod print;
pub mod save;
pub mod screen;
pub mod static_vec;
pub mod unsafe_utils;
pub mod utils;
pub mod version;

pub use mvutils_proc_macro::Savable;

#[cfg(test)]
mod tests {
    use crate::save::{Loader, Savable, Saver};
    use bytebuffer::ByteBuffer;
    use mvutils_proc_macro::Savable;

    #[derive(Savable, Debug)]
    enum E {
        A,
        B(String, u32, i32),
        C {
            a: String,
            #[unsaved]
            b: u32,
            c: i32,
        },
    }

    #[test]
    fn it_works() {
        let mut buffer = ByteBuffer::new();
        let e = E::C {
            a: "Hello".to_string(),
            b: 123,
            c: -123,
        };
        e.save(&mut buffer);
        println!("{:?}", buffer);
        let mut buffer = ByteBuffer::from_bytes(buffer.as_bytes());
        let e = E::load(&mut buffer).unwrap();
        println!("{:?}", e);
    }
}
