pub mod utils;
pub mod version;
pub mod args;
pub mod screen;
pub mod save;
pub mod unsafe_utils;
#[cfg(test)]
mod tests {
    use crate::unsafe_utils::UnsafeRef;

    #[derive(Debug)]
    struct A {
        a: i32
    }

    #[test]
    fn it_works() {
        let unsafe_ptr = gen();
        let data = &*unsafe_ptr;
        println!("{:?}", data);
    }

    fn gen() -> UnsafeRef<A> {
        let value = A { a: 10 };
        let unsafe_ptr = unsafe { UnsafeRef::new(&value) };
        unsafe_ptr
    }
}
