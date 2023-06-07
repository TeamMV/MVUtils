use std::collections::HashMap;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};
use std::ops::Range;
use std::panic::PanicInfo;
use std::sync::{LockResult, Mutex};
use std::time::*;
use num_traits::One;
use crate::lazy;
use crate::once::Lazy;

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

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + PartialOrd + Copy> Map<T> for T {
    fn map(self, original: &Range<T>, target: &Range<T>) -> T {
        if self < original.start || self > original.end { return self }
        ((self - original.start) * (target.end - target.start) / (original.end - original.start)) + target.start
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
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_else(|e| panic!("System clock error: Time elapsed of -{}ms is not valid!", e.duration().as_millis())).as_millis()
    }

    fn time_nanos() -> Self {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_else(|e| panic!("System clock error: Time elapsed of -{}ns is not valid!", e.duration().as_nanos())).as_nanos()
    }
}

pub trait IncDec {
    fn inc(&mut self) -> Self;
    fn post_inc(&mut self) -> Self;
    fn dec(&mut self) -> Self;
    fn post_dec(&mut self) -> Self;
}

impl<T: AddAssign + SubAssign + One + Copy> IncDec for T {
    fn inc(&mut self) -> Self {
        *self += T::one();
        *self
    }

    fn post_inc(&mut self) -> Self {
        let ret = *self;
        *self += T::one();
        ret
    }

    fn dec(&mut self) -> Self  {
        *self -= T::one();
        *self
    }

    fn post_dec(&mut self) -> Self {
        let ret = *self;
        *self -= T::one();
        ret
    }
}

#[macro_export]
macro_rules ! init_arr {
    ($len:literal, $item:expr) => {
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
            ($d($d t:ty$d([$d($d g:ident $d(:$d($d dep:ident),*)?),*])?),*) => {
                $d(
                    impl$d(<$d($d g$d(:$d($d dep+)*)?),*>)? private::Sealed for $d t {}
                )*
            };
            (
                $d(#[$d outer:meta])*
                $d vis:vis struct $d name:ident {
                    $d($d inner:tt)*
                }$d([$d($d g:ident $d(:$d($d dep:ident),*)?),*])?
            ) => {
                $d(#[$d outer])*
                $d vis struct $d name {
                    $d($d inner)*
                }

                impl$d(<$d($d g$d(:$d($d dep+)*)?),*>)? private::Sealed for $d name {}
            };
            (
                $d(#[$d outer:meta])*
                $d vis:vis struct $d name:ident(
                    $d($d inner:tt)*
                )$d([$d($d g:ident $d(:$d($d dep:ident),*)?),*])?
            ) => {
                $d(#[$d outer])*
                $d vis struct $d name(
                    $d($d inner)*
                )

                impl$d(<$d($d g$d(:$d($d dep+)*)?),*>)? private::Sealed for $d name {}
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

lazy! {
    static IDS: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

pub fn next_id(key: &str) -> u64 {
    let mut guard = IDS.lock().unwrap();
    let entry = guard.entry(key.to_string()).or_insert(0);
    *entry += 1;

    *entry
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

/// `fn_for` macro.
///
/// This macro is used to generate functions that can be called directly from wrappers around
/// a type, for example [`Arc<RwLock<T>>`]. Since you cannot use `self: Arc<RwLock<T>>`, you
/// can use this macro instead to automatically generate a trait and implementations for the
/// functions you need.
///
/// The macro also requires a token like `this` to replace `self` in the function body,
/// because `self` cannot be used directly in the macro invocation.
///
/// The macro is unable to generate a trait name, therefore you have to give it one. It will also
/// `seal!()` the trait to make it private (The functions keep the user-defined visibility). Because
/// of this, you must have called the `sealable!()` macro before using this macro.
///
/// The syntax for the macro is as follows:
///
/// ```ignore
/// fn_for!(Wrapper<Type> => visibility trait_name {
///     fn function_name(this, parameters) -> return_type {
///         function_body
///     }
/// });
/// ```
///
/// # Examples
///
/// Here's an example of using `fn_for` to generate a trait and its implementation for an `Arc<RwLock<A>>`:
///
/// ```rust
/// use std::sync::{Arc, RwLock};
/// use mvutils::{sealable, fn_for};
///
/// struct A {
///     i: i32,
/// }
///
/// sealable!();
///
/// fn_for!(Arc<RwLock<A>> => pub ATrait {
///     fn get(this, a: i32) -> i32 {
///         this.read().unwrap().i
///     }
/// });
/// ```
///
/// Note that `this` is used in the function body where `self` would normally be used.
///
/// # Notes
///
/// The `self` keyword cannot be used in the macro invocation itself. Instead, another token (in the examples, `this`) should be used.
///
/// Be sure to call `sealable!();` macro before invoking this macro to ensure the generation of a private sealed trait.
#[macro_export]
macro_rules! fn_for {
    (
        $(#[$outer:meta])*
        $t:ty => $v:vis $i:ident {
        $(
            $(#[$inner:meta])*
            fn $n:ident($this:ident, $($p:ident: $pt:ty),*) $(-> $r:ty)? $body:block
        )*
    }) => {
        sealed!(
        $(#[$outer:meta])*
        $v trait $i {
            $(
                $(#[$inner])*
                fn $n(&self, $($p: $pt),*) $(-> $r)?;
            )*
        }
        );

        seal!($t);

        $(#[$outer:meta])*
        impl $i for $t {
            $(
                $(#[$inner])*
                fn $n(&self, $($p: $pt),*) $(-> $r)? {
                    let $this = self;
                    $body
                }
            )*
        }
    };
}

pub trait Recover<T> {
    fn recover(self) -> T;
}

impl<T> Recover<T> for LockResult<T> {
    fn recover(self) -> T {
        self.unwrap_or_else(|r| r.into_inner())
    }
}

pub enum PanicStyle {
    Normal,
    ForceExit,
    Abort
}

pub fn setup_private_panic(panic_style: Option<PanicStyle>) {
    std::panic::set_hook(Box::new(match panic_style {
        Some(PanicStyle::ForceExit) => panic_force,
        Some(PanicStyle::Abort) => panic_abort,
        _ => panic
    }));
}

pub fn setup_private_panic_default() {
    setup_private_panic(None)
}

fn panic_force(info: &PanicInfo) {
    panic(info);
    std::process::exit(1)
}

fn panic_abort(info: &PanicInfo) {
    panic(info);
    std::process::abort()
}

fn panic(info: &PanicInfo) {
    let thread = std::thread::current().name().unwrap_or("unknown").to_string();
    if let Some(message) = info.payload().downcast_ref::<&'static str>() {
        println!("Thread '{}' panicked with message '{}'", thread, message);
    }
    else if let Some(message) = info.payload().downcast_ref::<String>() {
        println!("Thread '{}' panicked with message '{}'", thread, message);
    }
    else if let Some(message) = info.payload().downcast_ref::<std::fmt::Arguments>() {
        println!("Thread '{}' panicked with message '{}'", thread, message);
    }
    else {
        println!("Thread '{}' panicked", thread);
    }
}


pub fn key(mut n: u32) -> String {
    let mut result = String::new();
    loop {
        let remainder = (n % 26) as u8;
        result.push(('a' as u8 + remainder) as char);
        n /= 26;
        if n == 0 {
            break;
        }
        n -= 1;
    }
    result.chars().rev().collect()
}