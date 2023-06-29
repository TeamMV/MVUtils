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
    struct A {
        a: i32,
        #[unsaved]
        b: u32,
        c: String
    }


    #[test]
    fn it_works() {
        let a = A {
            a: 1,
            b: 2,
            c: "hello".to_string()
        };
        println!("{:?}", a);
        let mut buffer = ByteBuffer::new();
        a.save(&mut buffer);
        println!("{:?}", buffer.as_bytes());
        let b = A::load(&mut buffer).unwrap();
        println!("{:?}", b);
    }
}
