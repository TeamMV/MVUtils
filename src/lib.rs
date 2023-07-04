pub mod utils;
pub mod version;
pub mod screen;
pub mod save;
pub mod unsafe_utils;
pub mod print;
pub mod once;
pub mod static_vec;
pub mod args;
pub mod cinit;

#[cfg(test)]
mod tests {
    use bytebuffer::ByteBuffer;
    use mvutils_proc_macro::Savable;
    use crate::save::{Saver, Loader, Savable};

    #[derive(Savable, Debug)]
    enum E {
        A,
        B(String, u32, i32),
        C {
            a: String,
            #[unsaved]
            b: u32,
            c: i32
        }
    }


    #[test]
    fn it_works() {
        let mut buffer = ByteBuffer::new();
        let e = E::C {
            a: "Hello".to_string(),
            b: 123,
            c: -123
        };
        e.save(&mut buffer);
        println!("{:?}", buffer);
        let mut buffer = ByteBuffer::from_bytes(buffer.as_bytes());
        let e = E::load(&mut buffer).unwrap();
        println!("{:?}", e);


    }
}
