use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Div, Mul};
use std::time::*;
use num_traits::Num;

pub trait Plural {
    fn plural(&self, count: u32) -> Self;
    fn plural_irregular(&self, plural: Self, count: u32) -> Self;
}

impl Plural for String {
    fn plural(&self, count: u32) -> String {
        match count {
            1 => self.clone(),
            _ => self.clone() + "s",
        }
    }

    fn plural_irregular(&self, plural: String, count: u32) -> String {
        match count {
            1 => self.clone(),
            _ => plural,
        }
    }
}

pub trait ExtraFMath {
    fn percentage(self, total: Self) -> Self;
    fn value(self, total: Self) -> Self;
}

pub trait Square {
    fn square(self) -> Self;
}

impl<T: Mul<T, Output = T> + Copy> Square for T {
    fn square(self) -> Self {
        self * self
    }
}

pub trait Overlap {
    fn overlap(self, min: Self, max: Self) -> Self;
}

impl<T: Num + Ord + Copy> Overlap for T {
    fn overlap(self, min: T, max: T) -> Self {
        if self > max {
            return min + (self - max - T::one()) % (max - min + T::one());
        }
        else if self < min {
            return max - (min - self - T::one()) % (max - min + T::one());
        }
        else {
            self
        }
    }
}

pub trait Percentage {
    fn percentage(self, total: Self) -> Self;
    fn value(self, total: Self) -> Self;
}

impl<T: From<f32> + Div<T, Output = T> + Mul<T, Output = T> + Copy> Percentage for T {
    fn percentage(self, total: Self) -> Self {
        self / total * 100.0.into()
    }

    fn value(self, total: Self) -> Self {
        self / 100.0.into() * total
    }
}

pub trait TetrahedronOp {
    fn yn<T>(self, yes: T, no: T) -> T;
}

impl TetrahedronOp for bool {
    fn yn<T>(self, yes: T, no: T) -> T {
        if self { yes } else { no }
    }
}

#[macro_export]
macro_rules! try_catch {
    ($t:expr, $c:expr) => {
        match $t {
            Ok(v) => Some(v),
            Err(e) => {
                let val = $c(e);
                if let Some(ret) = val {
                    return ret;
                }
                None
            }
        }
    };
}

#[macro_export]
macro_rules! try_fn_catch {
    ($t:expr, $c:expr) => {
        match $t() {
            Ok(v) => Some(v),
            Err(e) => {
                let val = $c(e);
                if let Some(ret) = val {
                    return ret;
                }
                None
            }
        }
    };
}

/// Sometimes, in the try_fn_catch macro, the ? operator breaks and the return type is unknown.
/// This macro is a copy of the old try! macro, but for some reason I do not understand, this does
/// work.
#[macro_export]
macro_rules! ret_err {
    ($r:expr) => {
        match $t {
            Result::Ok(v) => v,
            Result::Err(e) => {
                return Result::Err(e);
            }
        }
    };
}

pub trait SplitInto {
    fn split_into(self, n: usize) -> Vec<Self> where Self: Sized;
}

impl<T> SplitInto for Vec<T> {
    fn split_into(mut self, n: usize) -> Vec<Self> {
        let len = self.len();
        assert!(n > 0);

        if n == 1 {
            return vec![self];
        }

        let mut parts = Vec::with_capacity(n);
        for _ in 0..n {
            parts.push(Vec::<T>::new());
        }

        if n >= len {
            for i in 0..len {
                if i < len {
                    parts[len - i - 1].push(self.pop().unwrap());
                }
            }
            return parts;
        }

        let split_data_length = self.len() / n;
        let mut extra = self.len() % n;
        self.reverse();

        for i in 0..n {
            let mut length = split_data_length;
            if extra > 0 {
                length += 1;
                extra -= 1;
            }
            for _ in 0..length {
                parts[i].push(self.pop().unwrap())
            }
        }

        return parts;
    }
}

pub trait Time {
    fn time_millis() -> Self;
    fn time_nanos() -> Self;
}

impl Time for u128 {
    fn time_millis() -> Self {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
    }

    fn time_nanos() -> Self {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos()
    }
}


pub trait IncDec {
    fn inc(self)  -> Self;
    fn dec(self) -> Self;
}

impl<T: Num + Copy> IncDec for T {
    fn inc(self) -> Self {
        self + T::one()
    }

    fn dec(self) -> Self {
        self - T::one()
    }
}

#[macro_export]
macro_rules ! init_arr {
    ($len:expr, $item:expr) => {
        [0; $len].map(|_| $item)
    };
}

pub trait SplitSized {
    fn split_sized(&self, n: usize) -> Vec<Self> where Self: Sized;
}

impl SplitSized for String {
    fn split_sized(&self, n: usize) -> Vec<Self> {
        assert!(n > 0);
        let mut vec = vec![];
        let mut buf = String::new();
        for c in self.chars() {
            buf.push(c);
            if buf.len() >= n {
                vec.push(buf);
                buf = String::new();
            }
        }
        vec
    }
}

pub type Bytecode = Vec<u8>;

pub trait Verify {
    fn verify(&self) -> bool;
    fn verify_or_panic(&self, message: &str) {
        if !self.verify() {
            panic!("{}", message);
        }
    }

    fn verify_or_panic_default(&self) {
        if !self.verify() {
            panic!("Illegal state, value verification returned false!");
        }
    }
}

#[macro_export]
macro_rules! inner_sealable {
    ($d:tt) => {
        use private::Sealed;

        mod private {
            pub trait Sealed {}
        }

        #[macro_export]
        macro_rules! seal {
            ($d($d t:ty),*) => {
                $d(
                    impl private::Sealed for $d t {}
                )*
            };
            (
                $d(#[$d outer:meta])*
                $d vis:vis struct $d name:ident {
                    $d($d inner:tt)*
                }
            ) => {
                $d(#[$d outer])*
                $d vis struct $d name {
                    $d($d inner)*
                }

                impl private::Sealed for $d name {}
            };
            (
                $d(#[$d outer:meta])*
                $d vis:vis struct $d name:ident;
            ) => {
                $d(#[$d outer])*
                $d vis struct $d name;

                impl private::Sealed for $d name {}
            };
        }

        #[macro_export]
        macro_rules! sealed {
            (
                $d(#[$d outer:meta])*
                $d vis:vis trait $d name:ident: $d a:ident $d(+$d t:ident)* {
                    $d($d inner:item)*
                }
            ) => {
                $d(#[$d outer])*
                $d vis trait $d name: Sealed + $d a $d(+$d t)* {
                    $d($d inner)*
                }
            };
            (
                $d(#[$d outer:meta])*
                $d vis:vis trait $d name:ident {
                    $d($d inner:item)*
                }
            ) => {
                $d(#[$d outer])*
                $d vis trait $d name: Sealed {
                    $d($d inner)*
                }
            };
        }
    };
}

#[macro_export]
macro_rules! sealable {
    () => {
        use $crate::inner_sealable;
        inner_sealable!($);
    };
}

#[macro_export]
macro_rules! unsafe_cast {
    ($val:ident, $to:ty) => {
        unsafe { (($val as *const _) as *const $to).as_ref().unwrap() }
    };
}

#[macro_export]
macro_rules! swap {
    ($a:ident, $b:ident) => {
        let tmp = $a;
        let $a = $b;
        let $b = tmp;
    };
}

struct L<T, F = fn() -> T> {
    val: Option<T>,
    gen: Option<F>
}

impl<T, F: FnOnce() -> T> L<T, F> {
    fn force(&mut self) -> &mut T {
        if let Some(ref mut val) = self.val {
            val
        }
        else if let Some(gen) = self.gen.take() {
            self.val = Some(gen());
            self.val.as_mut().unwrap()
        }
        else {
            panic!();
        }
    }
}

impl<T, F: FnOnce() -> T> Deref for L<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            (self as *const L<T, F>).cast_mut().as_mut().unwrap().force()
        }
    }
}

impl<T, F: FnOnce() -> T> DerefMut for L<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.force()
    }
}

static mut IDS: L<HashMap<String, u64>> = L { val: None, gen: Some(HashMap::new) };

pub fn next_id(key: &str) -> u64 {
    unsafe {
        if IDS.contains_key(key) {
            let id = IDS.get_mut(key).unwrap();
            *id += 1;
            id.clone()
        }
        else {
            IDS.insert(key.to_string(), 0);
            0
        }
    }
}