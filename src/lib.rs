pub mod hashers;
pub mod once;
pub mod print;
pub mod remake;
#[cfg(feature = "savable_arc")]
pub mod savable_arc;
pub mod save;
pub mod static_vec;
pub mod thread;
pub mod unsafe_utils;
pub mod utils;
pub mod version;
pub mod state;

pub use mvutils_proc_macro::{try_from_string, Savable};

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};
    use crate as mvutils;
    use bytebuffer::ByteBuffer;
    use mvutils_proc_macro::try_from_string;
    use mvutils_proc_macro::Savable;
    use crate::state::State;
    use crate::{update, when};
    use crate::save::{Savable, ShortString, ToShortString};

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
    fn test_derive_savable() {
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

    #[test]
    fn test_saving_time() {
        use crate::save::Savable;
        let a = SystemTime::now();
        sleep(Duration::from_millis(300));
        let b = SystemTime::now();

        println!("{}", b.duration_since(a).unwrap().as_millis());

        let mut buffer = ByteBuffer::new();

        a.save(&mut buffer);

        buffer.reset_cursors();

        let c = SystemTime::load(&mut buffer).unwrap();

        sleep(Duration::from_millis(100));

        println!("{}", b.duration_since(c).unwrap().as_millis());
    }

    #[test]
    fn test_state() {
        let state = State::new("Hello".to_string());
        state.force_outdated();

        let handle = {
            let state = state.clone();
            std::thread::spawn(move || {
                for _ in 0..10 {
                    when!([state] => {
                        println!("{}", state.read());
                    });
                    update!([state]);
                    sleep(Duration::from_millis(100));
                }
            })
        };

        sleep(Duration::from_millis(200));
        state.write().push_str(", world!");

        handle.join().unwrap();
    }

    #[test]
    fn test_short_string() {
        let short_string: ShortString = "Hello".to_short_string();

        assert!(short_string.validate());

        let mut buffer = ByteBuffer::new();
        short_string.save(&mut buffer);

        assert_eq!(buffer.len(), 6);

        let str = ShortString::load(&mut buffer).unwrap();

        assert_eq!(short_string, str);

        let mut long_string: ShortString = "".to_short_string();
        for i in 0..260 {
            long_string.push('c');
        }

        assert!(!long_string.validate());

        let mut buffer = ByteBuffer::new();
        long_string.save(&mut buffer);

        assert_eq!(buffer.len(), 256);

        let str = ShortString::load(&mut buffer).unwrap();

        assert_ne!(short_string, str);

        let mut test_str = "".to_short_string();
        for i in 0..255 {
            test_str.push('c');
        }

        assert_eq!(str, test_str);
    }
}
