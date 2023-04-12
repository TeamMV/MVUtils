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
        {
            match $t {
                Ok(v) => Some(v),
                Err(e) => {
                    $c(e);
                    None
                }
            }
        }
    };
}