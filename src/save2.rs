use crate::save::{Loader, Savable, Saver};
use bytebuffer::ByteBuffer;
use crate::save::custom::load;

pub trait Savable2 {
    fn save(&self, saver: &mut ByteBuffer);
    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String>;
}

impl<T: Savable> Savable2 for T {
    fn save(&self, saver: &mut ByteBuffer) {
        Savable::save(self, saver);
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        let l = T::load(loader)?;
        *self = l;
        Ok(())
    }
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