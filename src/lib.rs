#[macro_use]
extern crate log;


mod event_queue;
mod layer;

#[cfg(not(feature = "tokio"))]
mod locked_reg;

#[cfg(feature = "tokio")]
mod locked_reg_async;

use std::any::TypeId;
use std::collections::HashMap;

use crate::layer::AnyLayer;

pub use crate::event_queue::EventQueue;
pub use layer::{Layer, LayerDispatch};

#[cfg(not(feature = "tokio"))]
pub use crate::locked_reg::LockedReg;

#[cfg(feature = "tokio")]
pub use crate::locked_reg_async::LockedReg;

pub struct LayerReg<E: Send + Sync>
{
    layers: HashMap<TypeId, AnyLayer<E>>,
}


impl<E: Send + Sync> LayerReg<E>
{
    pub fn new() -> Self
    {
        Self {
            layers: HashMap::new(),
        }
    }

    pub fn insert_any(&mut self, layer: AnyLayer<E>) -> Option<AnyLayer<E>>
    {
        if self.layers.contains_key(&layer.id())
        {
            return Some(layer);
        }

        self.layers.insert(layer.id(), layer);

        None
    }

    pub fn insert<T>(&mut self, layer: T) -> Option<T>
    where
        T: LayerDispatch<E> + Send + Sync + 'static,
    {
        let key = TypeId::of::<T>();

        if self.layers.contains_key(&key)
        {
            return Some(layer);
        }

        let layer = Layer::new(layer);
        self.layers.insert(key, layer.into());

        None
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Layer<T>>
    {
        self.layers
            .get(&TypeId::of::<T>())
            .map(|l| Layer::try_from(l).unwrap())
    }

    pub fn get_unchecked<T: Send + Sync + 'static>(&self) -> Layer<T>
    {
        self.get().unwrap()
    }

    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<Layer<T>>
    {
        self.layers
            .remove(&TypeId::of::<T>())
            .map(|l| Layer::try_from(&l).unwrap())
    }

    #[cfg(not(feature = "tokio"))]
    pub fn dispatch(&mut self, event: E) -> EventQueue<E>
    {
        let mut queue = EventQueue::default();

        for layer in self.layers.values_mut()
        {
            layer.dispatch(&event, &mut queue);
        }

        queue
    }

    #[cfg(feature = "tokio")]
    pub async fn dispatch(&mut self, event: E) -> EventQueue<E>
    {
        let mut queue = EventQueue::default();

        for layer in self.layers.values_mut()
        {
            layer.dispatch(&event, &mut queue).await;
        }

        queue
    }
}


impl<E: Send + Sync> Default for LayerReg<E>
{
    fn default() -> Self
    {
        Self::new()
    }
}


#[macro_export]
macro_rules! reg_inspect {
    ($reg:expr, $name:ident = $layer:ident => $f:expr) => {{
        if let Some(layer) = $reg.get::<$layer>()
        {
            #[allow(unused_mut)]
            if let Ok(mut $name) = layer.write()
            {
                $f;
            }
        }
    }};
}


#[macro_export]
macro_rules! layer_inspect {
    ($name:ident = $layer:expr => $f:expr) => {{
        #[allow(unused_mut)]
        if let Ok(mut $name) = $layer.write()
        {
            $f
        }
    }};
}


#[cfg(not(feature = "tokio"))]
#[macro_export]
macro_rules! layer_read {
    ($layer:expr) => {
        $layer.read().unwrap()
    };
}


#[cfg(not(feature = "tokio"))]
#[macro_export]
macro_rules! layer_write {
    ($layer:expr) => {
        $layer.write().unwrap()
    };
}


#[cfg(not(feature = "tokio"))]
#[macro_export]
macro_rules! reg_read {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().read().unwrap()
    };
}


#[cfg(not(feature = "tokio"))]
#[macro_export]
macro_rules! reg_write {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().write().unwrap()
    };
}


#[cfg(not(feature = "tokio"))]
#[macro_export]
macro_rules! locked_read {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().read().unwrap()
    };
}


#[cfg(not(feature = "tokio"))]
#[macro_export]
macro_rules! locked_write {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().write().unwrap()
    };
}


#[cfg(feature = "tokio")]
#[macro_export]
macro_rules! layer_read {
    ($layer:expr) => {
        $layer.read().await
    };
}


#[cfg(feature = "tokio")]
#[macro_export]
macro_rules! layer_write {
    ($layer:expr) => {
        $layer.write().await
    };
}


#[cfg(feature = "tokio")]
#[macro_export]
macro_rules! reg_read {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().read().await
    };
}


#[cfg(feature = "tokio")]
#[macro_export]
macro_rules! reg_write {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().write().await
    };
}


#[cfg(feature = "tokio")]
#[macro_export]
macro_rules! locked_read {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().await.read().await
    };
}


#[cfg(feature = "tokio")]
#[macro_export]
macro_rules! locked_write {
    ($reg:expr, $layer:ident) => {
        $reg.get_unchecked::<$layer>().await.write().await
    };
}
