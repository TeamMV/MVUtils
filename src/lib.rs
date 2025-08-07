#![feature(array_try_from_fn)]

pub mod args;
pub mod hashers;
pub mod once;
pub mod print;
pub mod remake;
pub mod save;
pub mod static_vec;
pub mod thread;
pub mod unsafe_utils;
pub mod utils;
pub mod version;
pub mod state;
pub mod bytebuffer;
pub mod clock;

#[cfg(feature = "savable_arc")]
pub mod savable_arc;
mod alloc;

pub use mvutils_proc_macro::{TryFromString, Savable};

#[cfg(test)]
#[allow(dead_code)]
mod tests {
use std::ops::{Deref, DerefMut};
    use std::str::FromStr;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};
    use crate as mvutils;
    use bytebuffer::ByteBuffer;
    use mvutils_proc_macro::{TryFromString};
    use mvutils_proc_macro::Savable;
    use crate::state::State;
    use crate::{update, when};
    use crate::save::{Loader, Savable, Saver};

    #[derive(Savable)]
    struct A;

    #[derive(Savable)]
    struct B(String, #[unsaved] u32, #[custom(save = hello, load = world)] i32);

    #[derive(Savable)]
    struct C {
        a: String,
        #[unsaved]
        _b: u32,
        #[custom(save = hello, load = world)]
        c: i32,
    }

    #[derive(Savable)]
    struct D {
        a: String,
        #[unsaved]
        _b: u32,
        #[custom(save = hello, load = world)]
        c: i32,
        d: u32,
        #[unsaved]
        e: String,
        f: u32,
        #[custom(save = hello, load = world)]
        g: i32,
    }

    fn hello(saver: &mut impl Saver, val: &i32) {
        saver.push_i32(*val);
        saver.push_i32(*val);
        saver.push_i32(*val);
    }

    fn world(loader: &mut impl Loader) -> Result<i32, String> {
        let a = i32::load(loader)?;
        let b = i32::load(loader)?;
        let c = i32::load(loader)?;
        if a == b {
            Ok(a)
        } else if b == c {
            Ok(b)
        } else if a == c {
            Ok(c)
        } else {
            Err("Error loading i32: all values corrupted!".to_string())
        }
    }

    #[derive(Savable, Debug)]
    #[varint]
    enum E {
        A,
        B(String, #[unsaved] u32, #[custom(save = hello, load = world)] i32),
        C {
            a: String,
            #[unsaved]
            _b: u32,
            #[custom(save = hello, load = world)]
            c: i32,
        },
    }

    #[derive(Debug, TryFromString)]
    enum Enum {
        A,
        B,
        C,
        #[exclude]
        HelloWorld,
    }

    #[test]
    fn test_try_from_string() {
        let a = Enum::from_str("a").unwrap();
        println!("{:?}", a);
        let b = Enum::from_str("B").unwrap();
        println!("{:?}", b);
        let hello = Enum::from_str("HelloWorld");
        assert!(hello.is_err());
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

    use mvutils::save::custom::{string8_load, string8_save};
    use crate::bytebuffer::ByteBufferExtras;

    #[derive(Savable, Debug, Eq, PartialEq)]
    struct ShortString(
        #[custom(save = string8_save, load = string8_load)]
        String
    );

    impl Deref for ShortString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for ShortString {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl From<String> for ShortString {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    #[test]
    fn test_short_string() {
        let short_string: ShortString = "Hello".to_string().into();

        let mut buffer = ByteBuffer::new();
        short_string.save(&mut buffer);

        assert_eq!(buffer.len(), 6);

        let str = ShortString::load(&mut buffer).unwrap();

        assert_eq!(short_string, str);

        let mut long_string: ShortString = "".to_string().into();
        for _ in 0..260 {
            long_string.push('c');
        }

        let mut buffer = ByteBuffer::new();
        long_string.save(&mut buffer);

        assert_eq!(buffer.len(), 256);

        let str = ShortString::load(&mut buffer).unwrap();

        assert_ne!(short_string, str);

        let mut test_str: ShortString = "".to_string().into();
        for _ in 0..255 {
            test_str.push('c');
        }

        assert_eq!(str, test_str);
    }

    #[test]
    fn test_array() {
        let mut test = [0; 15];
        for i in 0..15 {
            test[i] = i as i32 + 1;
        }

        let mut buffer = ByteBuffer::new();
        test.save(&mut buffer);

        assert_eq!(buffer.len(), 15 * 4);

        let test2 = <[i32; 15]>::load(&mut buffer).unwrap();
        assert_eq!(test2, test);
    }

    #[test]
    fn test_bytebuffer_features() {
        let mut buffer = ByteBuffer::new_le();
        buffer.push_u16(0x1000);
        assert_eq!(buffer.as_bytes(), &[0x00, 0x10]);

        let mut buffer = ByteBuffer::from_vec_le(buffer.into_vec());
        assert_eq!(buffer.pop_u16(), Some(0x1000));
    }
}
