use crate::save::{Loader, Savable, Saver};
use bytebuffer::ByteBuffer;

pub trait Savable2 {
    fn save(&self, saver: &mut ByteBuffer);
    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String>;
}

pub trait Savable2Convenience: Sized {
    fn load_static(loader: &mut ByteBuffer) -> Result<Self, String>;
}

impl<T: Savable2 + Default> Savable2Convenience for T {
    fn load_static(loader: &mut ByteBuffer) -> Result<Self, String> {
        let mut d = T::default();
        d.load_into(loader)?;
        Ok(d)
    }
}

macro_rules! impl_savable2_primitive {
    ($($t:ty, $pu:ident, $po:ident),*) => {
        $(
            impl Savable2 for $t {
                fn save(&self, saver: &mut ByteBuffer) {
                    saver.$pu(*self)
                }

                fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
                    *self = loader.$po().ok_or(format!("Failed to load {} from Loader!", stringify!($t)))?;
                    Ok(())
                }
            }
        )*
    };
}

impl_savable2_primitive!(
    bool, push_bool, pop_bool, u8, push_u8, pop_u8, u16, push_u16, pop_u16, u32, push_u32, pop_u32,
    u64, push_u64, pop_u64, i8, push_i8, pop_i8, i16, push_i16, pop_i16, i32, push_i32, pop_i32,
    i64, push_i64, pop_i64, f32, push_f32, pop_f32, f64, push_f64, pop_f64
);

impl Savable2 for usize {
    fn save(&self, saver: &mut ByteBuffer) {
        let mut value = *self;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            saver.push_u8(byte);
            if value == 0 {
                break;
            }
        }
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        let mut result: usize = 0;
        let mut shift = 0;
        let usize_bits = size_of::<usize>() * 8;

        loop {
            if shift >= usize_bits {
                return Err("Failed to load usize: Incompatible system bitness".to_string());
            }
            let byte = u8::load(loader)?;
            result |= ((byte & 0x7F) as usize) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        *self = result;
        Ok(())
    }
}

impl Savable2 for isize {
    fn save(&self, saver: &mut ByteBuffer) {
        let bits = size_of::<isize>() * 8;
        let zigzag = ( (*self as usize) << 1 ) ^ (((*self) >> (bits - 1)) as usize);
        let mut value = zigzag;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            saver.push_u8(byte);
            if value == 0 {
                break;
            }
        }
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        let mut result: usize = 0;
        let mut shift = 0;
        let usize_bits = size_of::<usize>() * 8;

        loop {
            if shift >= usize_bits {
                return Err("Failed to load isize: Incompatible system bitness".to_string());
            }
            let byte = u8::load(loader)?;
            result |= ((byte & 0x7F) as usize) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        let value = ((result >> 1) as isize) ^ (-((result & 1) as isize));
        *self = value;
        Ok(())
    }
}

macro_rules! impl_savable2_tuple {
    () => {};
    ($first:ident $($rest:ident)*) => {
        impl_savable2_tuple!($($rest)*);
        impl<$first: Savable2, $($rest: Savable2),*> Savable2 for ($first, $($rest),*) {
            fn save(&self, saver: &mut ByteBuffer) {
                #[allow(non_snake_case)]
                let ($first, $($rest),*) = self;
                $first.save(saver);
                $( $rest.save(saver); )*
            }

            fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
                #[allow(non_snake_case)]
                let ($first, $($rest),*) = self;
                $first.load_into(loader)?;
                $( $rest.load_into(loader)?; )*
                Ok(())
            }
        }
    };
}

impl_savable2_tuple!(E D C B A Z Y X W V U T);

impl Savable2 for String {
    fn save(&self, saver: &mut ByteBuffer) {
        saver.push_string(self);
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        *self = loader
            .pop_string()
            .ok_or("Failed to load String from Loader!".to_string())?;
        Ok(())
    }
}

impl<T: Savable2 + Default> Savable2 for Option<T> {
    fn save(&self, saver: &mut ByteBuffer) {
        match self {
            None => saver.push_u8(0),
            Some(t) => {
                saver.push_u8(1);
                t.save(saver);
            }
        }
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        if u8::load(loader)? == 1 {
            //god i hate this
            let mut d = T::default();
            d.load_into(loader)?;
            *self = Some(d);
        }
        Ok(())
    }
}

impl<T: Savable2 + Default, E: Savable2 + Default> Savable2 for Result<T, E> {
    fn save(&self, saver: &mut ByteBuffer) {
        match self {
            Ok(t) => {
                saver.push_u8(0);
                t.save(saver);
            }
            Err(e) => {
                saver.push_u8(1);
                e.save(saver);
            }
        }
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        match u8::load(loader)? {
            0 => {
                let mut d = T::default();
                d.load_into(loader)?;
                *self = Ok(d);
            },
            1 => {
                let mut d = E::default();
                d.load_into(loader)?;
                *self = Err(d);
            },
            _ => return Err("Failed to load Result from Loader!".to_string()),
        };
        Ok(())
    }
}

//continue to impl Savable2