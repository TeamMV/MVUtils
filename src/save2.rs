use crate::save::Savable;
use bytebuffer::ByteBuffer;

pub trait Savable2 {
    fn save(&self, saver: &mut ByteBuffer);
    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String>;
}

impl<T: Savable + Default> Savable2 for T {
    fn save(&self, saver: &mut ByteBuffer) {
        Savable::save(self, saver);
    }

    fn load_into(&mut self, loader: &mut ByteBuffer) -> Result<(), String> {
        let l = T::load(loader)?;
        *self = l;
        Ok(())
    }
}