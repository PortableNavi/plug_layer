use std::sync::RwLock;

use crate::{EventQueue, Layer, LayerDispatch, LayerReg};

#[derive(Default)]
pub struct LockedReg<T: Send + Sync>(RwLock<LayerReg<T>>);


impl<E: Send + Sync> LockedReg<E>
{
    pub fn new() -> Self
    {
        Self(RwLock::new(LayerReg::new()))
    }

    pub fn insert<T>(&self, layer: T) -> Option<T>
    where
        T: LayerDispatch<E> + Send + Sync + 'static,
    {
        self.0.write().unwrap().insert(layer)
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Layer<T>>
    {
        self.0.read().unwrap().get()
    }

    pub fn get_unchecked<T: Send + Sync + 'static>(&self) -> Layer<T>
    {
        self.0.read().unwrap().get_unchecked()
    }

    pub fn remove<T: Send + Sync + 'static>(&self) -> Option<Layer<T>>
    {
        self.0.write().unwrap().remove()
    }

    pub fn dispatch(&self, event: E) -> EventQueue<E>
    {
        self.0.write().unwrap().dispatch(event)
    }
}
