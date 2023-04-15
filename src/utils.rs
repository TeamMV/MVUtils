use crate::*;

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

pub trait XTraIMath {
    fn overlap(self, min: Self, max: Self) -> Self;
}

pub trait XTraFMath {
    fn percentage(self, total: Self) -> Self;
    fn value(self, total: Self) -> Self;
}

pub trait XTraMath {
    fn square(self) -> Self;
}

impl_xtraimath!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_xtrafmath!(f32, f64);
impl_xtramath!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[macro_export]
macro_rules! impl_xtraimath {
    ($($t:ty),*) => {
        $(
            impl XTraIMath for $t {
                fn overlap(self, min: $t, max: $t) -> $t {
                    if self > max {
                        return min + (self - max - 1) % (max - min + 1);
                    }
                    else if self < min {
                        return max - (min - self - 1) % (max - min + 1);
                    }
                    else {
                        self
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_xtrafmath {
    ($($t:ty),*) => {
        $(
            impl XTraFMath for $t {
                fn percentage(self, total: $t) -> $t {
                    self / total * 100.0
                }

                fn value(self, total: $t) -> $t {
                    self / 100.0 * total
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_xtramath {
    ($($t:ty),*) => {
        $(
            impl XTraMath for $t {
                fn square(self) -> $t {
                    self * self
                }
            }
        )*
    };
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