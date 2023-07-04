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
pub mod group_step_by;

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

    #[derive(Savable, Debug)]
    struct B(i32, u32, String);

    #[derive(Savable, Debug)]
    struct C;


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
        let a = A::load(&mut buffer).unwrap();
        println!("{:?}", a);

        println!();

        let b = B(1, 2, "hello".to_string());
        println!("{:?}", b);
        let mut buffer = ByteBuffer::new();
        b.save(&mut buffer);
        println!("{:?}", buffer.as_bytes());
        let b = B::load(&mut buffer).unwrap();
        println!("{:?}", b);

        println!();

        let c = C;
        println!("{:?}", c);
        let mut buffer = ByteBuffer::new();
        c.save(&mut buffer);
        println!("{:?}", buffer.as_bytes());
        let c = C::load(&mut buffer).unwrap();
        println!("{:?}", c);
    }
}
