use tokio::sync::RwLock;

use crate::{EventQueue, Layer, LayerDispatch, LayerReg};

#[derive(Default)]
pub struct LockedReg<T: Send + Sync>(RwLock<LayerReg<T>>);


impl<E: Send + Sync> LockedReg<E>
{
    pub fn new() -> Self
    {
        Self(RwLock::new(LayerReg::new()))
    }

    pub async fn insert<T>(&self, layer: T) -> Option<T>
    where
        T: LayerDispatch<E> + Send + Sync + 'static,
    {
        self.0.write().await.insert(layer)
    }

    pub async fn get<T: Send + Sync + 'static>(&self) -> Option<Layer<T>>
    {
        self.0.read().await.get()
    }

    pub async fn get_unchecked<T: Send + Sync + 'static>(&self) -> Layer<T>
    {
        self.0.read().await.get_unchecked()
    }

    pub async fn remove<T: Send + Sync + 'static>(&self) -> Option<Layer<T>>
    {
        self.0.write().await.remove()
    }

    pub async fn dispatch(&self, event: E) -> EventQueue<E>
    {
        self.0.write().await.dispatch(event).await
    }
}
