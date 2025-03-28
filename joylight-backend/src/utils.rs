//! Utility definitions

use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, RwLockReadGuard, Weak};

use anyhow::Result;
use uuid::Uuid;

/// An entity that has a UUID
pub trait WithUuid {
    /// Fetch the UUID without any hassle
    fn uuid(&self) -> Uuid;
}

/// A thread-safe smart pointer to an entity with a UUID
///
/// This is essentially a wrapper around `Arc<RwLock<T>>`
///
/// TODO: Think about implementing a `Weak` version?
pub struct SmartRef<T> {
    pub uuid: Uuid,
    value: Arc<RwLock<T>>,
}

impl<T> SmartRef<T> {
    pub fn new_with_uuid(value: Arc<RwLock<T>>, uuid: Uuid) -> SmartRef<T> {
        // let value = Arc::downgrade(&value);

        SmartRef { uuid, value }
    }

    /// Get the UUID of the stored entity. This is guaranteed to be a cheap operation.
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get(&self) -> Option<Arc<RwLock<T>>> {
        Some(self.value.clone())
    }

    /// Apply a function (that may return something) to a read-only entity.
    ///
    /// If the reference is not available or cannot be achieved due to other errors,
    /// an error will be returned and logged.
    pub fn read<F, R>(&self, f: F) -> Result<R>
    where
        F: Fn(&T) -> R,
    {
        //TODO: Log errors
        let arc = Some(&self.value)
            .ok_or_else(|| anyhow::anyhow!("Attempted to read entity {} which has been removed", self.uuid))?;

        let guard = arc.read().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(f(&guard))
    }

    /// Apply a function (that may return something) to the mutable entity.
    ///
    /// If the reference is not available or cannot be achieved due to other errors,
    /// an error will be returned and logged.
    pub fn write<F, R>(&self, f: F) -> Result<R>
    where
        F: Fn(&mut T) -> R,
    {
        let arc = Some(&self.value)
            .ok_or_else(|| anyhow::anyhow!("Attempted to edit entity {} which has been removed", self.uuid))?;

        let mut guard = arc.write().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(f(&mut guard))
    }
}

impl<T> SmartRef<T>
where
    T: WithUuid,
{
    pub fn new(value: Arc<RwLock<T>>) -> Result<SmartRef<T>> {
        let uuid = value
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .map(|v| v.uuid())?;

        Ok(SmartRef::new_with_uuid(value, uuid))
    }

    pub fn new_from_move(value: T) -> SmartRef<T> {
        let uuid = value.uuid();
        let value = Arc::new(RwLock::new(value));

        Self::new_with_uuid(value, uuid)
    }
}

impl<T> fmt::Debug for SmartRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmartRef<{}> {{ uuid: {} }}", std::any::type_name::<T>(), self.uuid)
    }
}

impl<T> Clone for SmartRef<T> {
    fn clone(&self) -> Self {
        SmartRef {
            uuid: self.uuid,
            value: self.value.clone(),
        }
    }
}

impl<T> PartialEq for SmartRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> PartialOrd for SmartRef<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.uuid.cmp(&other.uuid))
    }
}

impl<T> Eq for SmartRef<T> {}

impl<T> Hash for SmartRef<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}
