use std::collections::HashMap;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub};
use std::ops::Range;
use std::time::*;
use num_traits::One;

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

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Rem<T, Output = T> + One + Ord + Copy> Overlap for T {
    fn overlap(self, min: T, max: T) -> Self {
        if self > max {
            min + (self - max - T::one()) % (max - min + T::one())
        }
        else if self < min {
            max - (min - self - T::one()) % (max - min + T::one())
        }
        else {
            self
        }
    }
}

pub trait Map<T> {
    fn map(self, original: &Range<T>, target: &Range<T>) -> T;
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + PartialOrd> Map<T> for T {
    fn map(self, original: &Range<T>, target: &Range<T>) -> T {
        if self < original.start || self > original.end { return self }
        return ((self - original.start) * (target.end - target.start) / (original.end - original.start)) + target.start;
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

        for part in parts.iter_mut() {
            let mut length = split_data_length;
            if extra > 0 {
                length += 1;
                extra -= 1;
            }
            for _ in 0..length {
                part.push(self.pop().unwrap())
            }
        }

        parts
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
    fn inc(&mut self) -> Self;
    fn dec(&mut self) -> Self;
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + AddAssign + SubAssign + One + Copy> IncDec for T {
    fn inc(&mut self) -> Self {
        *self += T::one();
        *self
    }

    fn dec(&mut self) -> Self {
        *self -= T::one();
        *self
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
            *id
        }
        else {
            IDS.insert(key.to_string(), 0);
            0
        }
    }
}

#[macro_export]
macro_rules! id_eq {
    ($($t:ty $([$($g:ident$(:$($dep:ident),*)?),*])?),*) => {
        $(
            impl$(<$($g$(:$($dep+)*)?),*>)? PartialEq for $t {
                fn eq(&self, other: &$t) -> bool {
                    self.id == other.id
                }
            }

            impl$(<$($g$(:$($dep+)*)?),*>)? Eq for $t {}
        )*
    };
}

#[macro_export]
macro_rules! id {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$inner:meta])*
                $v:vis $n:ident: $t:ty
            ),*
        }
    ) => {
        $($outer)*
        $vis struct $name {
            id: u64,
            $(
                $(#[$inner])*
                $v $n: $t
            ),*
        }

        id_eq!($name);
    };

    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident<$($g:ident$(:$($dep:ident),*)?),*> {
            $(
                $(#[$inner:meta])*
                $v:vis $n:ident: $t:ty
            ),*
        }
    ) => {
        $($outer)*
        $vis struct $name<$($g$(:$($dep+)*)?),*> {
            id: u64,
            $(
                $(#[$inner])*
                $v $n: $t
            ),*
        }

        id_eq!($name<$($g),*>[$($g$(:$($dep),*)?),*]);
    }
}

#[macro_export]
macro_rules! attach_id {
    (
        $name:ident {
            $(
                $(#[$outer:meta])*
                $n:ident$(: $t:expr)?
            ),*
        }
    ) => {
        $name {
            id: mvutils::utils::next_id(stringify!($name)),
            $(
                $(#[$outer])*
                $n$(: $t)?
            ),*
        }
    };
}

pub fn remove_quotes(input: &str) -> String {
    let input = input.replace("' '", "'\\s'");
    let mut output = String::new();
    let mut in_quotes = false;
    let mut prev_char = '\0';

    for c in input.chars() {
        match c {
            '"' if prev_char != '\\' => {
                in_quotes = !in_quotes;
                continue;
            }
            ' ' if in_quotes => {
                output.push_str("\\s");
                prev_char = c;
                continue;
            }
            _ => {}
        }

        if !(prev_char == '\\' && c == '"') {
            output.push(c);
        }
        prev_char = c;
    }

    output
}

pub fn format_escaped(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('f') => {
                    output.push('\x0C');
                }
                Some('b') => {
                    output.push('\x08');
                }
                Some('s') => {
                    output.push(' ');
                }
                Some('r') => {
                    output.push('\r');
                }
                Some('n') => {
                    output.push('\n');
                }
                Some('t') => {
                    output.push('\t');
                }
                Some('0') => {
                    output.push('\0');
                }
                Some('\\') => {
                    output.push('\\');
                }
                Some(c) => {
                    output.push(*c);
                }
                None => {
                    panic!("Character after escape sequence not found!");
                }
            }
            chars.next();
        } else {
            output.push(c);
        }
    }
    output
}