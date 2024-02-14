use std::sync::Arc;
use hashbrown::HashMap;
use crate::save::{Loader, Savable, Saver};

pub trait Id {
    fn get_id(&self) -> u64;
}

pub trait StaticallyLoaded {
    fn get_map() -> &'static mut HashMap<u64, Arc<Self>>;
}

impl<T: Id + StaticallyLoaded + 'static> Savable for Arc<T> {
    fn save(&self, saver: &mut impl Saver) {
        saver.push_u64(self.get_id())
    }

    fn load(loader: &mut impl Loader) -> Result<Self, String> {
        Ok(T::get_map().get(&u64::load(loader)?).ok_or("Invalid ID".to_string())?.clone())
    }
}

#[macro_export]
macro_rules! savable_arc {
    ($($ty:ty => ($map:stmt,$id:ident))*) => {
        $(
            impl Id for $ty {
                fn get_id(&self) -> u64 {
                    self.$id
                }
            }

            impl StaticallyLoaded for $ty {
                fn get_map() -> &'static mut HashMap<u64, Arc<Self>> {
                    $map
                }
            }
        )*
    };
}