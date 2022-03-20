//! An abstraction around [HashMap], where the types are the keys.

use {
    std::{
        any::{Any, TypeId},
        collections::HashMap,
        sync::Arc,
    },
    tokio::sync::RwLock,
};

type AnyMap = HashMap<TypeId, Box<dyn Any + Send + Sync>>;

/// An abstraction around [HashMap], where the types are the keys,
/// can be used to store databases, counters, etc.
#[derive(Debug, Clone, Default)]
pub struct State(Arc<RwLock<AnyMap>>);

impl State {
    /// Get a value from the [State].
    ///
    /// The value will be cloned.
    pub async fn get<T>(&self) -> Option<T>
    where
        T: Clone + 'static,
    {
        self.0
            .read()
            .await
            .get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref::<T>().cloned())
    }

    /// Inserts a value by a type in the [State].
    pub async fn insert<T>(&self, value: T) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        self.0
            .write()
            .await
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|value| value.downcast::<T>().ok())
            .map(|value| *value)
    }

    /// Update a value in the [State].
    pub async fn update<T: 'static>(&self, update: impl Fn(&mut T)) {
        if let Some(value) = self
            .0
            .write()
            .await
            .get_mut(&TypeId::of::<T>())
            .and_then(|value| value.downcast_mut::<T>())
        {
            update(value);
        }
    }

    /// Check if there is a type in the [State].
    pub async fn contains<T: 'static>(&self) -> bool {
        self.0.read().await.contains_key(&TypeId::of::<T>())
    }

    /// Removes a value by a type in the [State].
    pub async fn remove<T: 'static>(&self) -> Option<T> {
        self.0
            .write()
            .await
            .remove(&TypeId::of::<T>())
            .and_then(|value| value.downcast::<T>().ok())
            .map(|value| *value)
    }
}
