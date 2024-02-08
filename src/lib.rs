pub mod once;
pub mod print;
pub mod save;
pub mod static_vec;
pub mod thread;
pub mod unsafe_utils;
pub mod utils;
pub mod version;
#[cfg(feature = "xml")]
pub mod xml;

pub use mvutils_proc_macro::{try_from_string, Savable};

#[cfg(test)]
mod tests {
    use crate as mvutils;
    use bytebuffer::ByteBuffer;
    use mvutils_proc_macro::try_from_string;
    use mvutils_proc_macro::Savable;

    #[derive(Savable)]
    struct A;

    #[derive(Savable)]
    struct B(String, u32, i32);

    #[derive(Savable)]
    struct C {
        a: String,
        #[unsaved]
        _b: u32,
        c: i32,
    }

    #[derive(Savable, Debug)]
    enum E {
        A,
        B(String, u32, i32),
        C {
            a: String,
            #[unsaved]
            _b: u32,
            c: i32,
        },
    }

    #[try_from_string]
    enum Enum {
        A,
        B,
        C,
        HelloWorld,
    }

    #[test]
    fn it_works() {
        use crate::save::Savable;
        let mut buffer = ByteBuffer::new();
        let e = E::C {
            a: "Hello".to_string(),
            _b: 123,
            c: -123,
        };
        e.save(&mut buffer);
        println!("{:?}", buffer);
        let mut buffer = ByteBuffer::from_bytes(buffer.as_bytes());
        let e = E::load(&mut buffer).unwrap();
        println!("{:?}", e);
    }
}
