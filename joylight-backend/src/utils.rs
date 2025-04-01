//! Utility definitions

use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{CoerceUnsized, Deref};
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
pub struct SmartRef<T: ?Sized> {
    uuid: Uuid,
    value: Arc<RwLock<T>>,
}

impl<T> SmartRef<T>
where
    T: ?Sized,
{
    pub fn new_with_uuid(value: Arc<RwLock<T>>, uuid: Uuid) -> SmartRef<T> {
        // let value = Arc::downgrade(&value);

        SmartRef { uuid, value }
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

    /// Creates a new SmartRef, without assuming that the value has a UUID internally. Instead, a UUID
    /// is generated and stored in the reference.
    pub fn new_from_move_without_uuid<U>(value: U) -> SmartRef<T>
    where
        Arc<RwLock<U>>: CoerceUnsized<Arc<RwLock<T>>>,
    {
        let uuid = Uuid::new_v4();
        let arc = Arc::new(RwLock::new(value));

        Self::new_with_uuid(arc, uuid)
    }
}

impl<T> SmartRef<T> {}

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

impl<T> WithUuid for SmartRef<T> {
    /// Get the UUID of the stored entity. This is guaranteed to be a cheap operation.
    fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl<T> fmt::Debug for SmartRef<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmartRef<{}> {{ {} }}", std::any::type_name::<T>(), self.uuid)
        // let mut dbg = f.debug_struct(format!("SmartRef<{}>", std::any::type_name::<T>()).as_str());

        // if let Ok(guard) = self.value.read() {
        // dbg.field("value", &guard)
        // } else {
        // dbg.field("value", &"Error reading value")
        // }.finish()
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

impl<T> Ord for SmartRef<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<T> PartialOrd for SmartRef<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for SmartRef<T> {}

impl<T> Hash for SmartRef<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

/// A Weak version of [SmartRef], implemented using [Weak] instead of [Arc].
pub struct WeakRef<T: ?Sized> {
    uuid: Uuid,
    value: Weak<RwLock<T>>,
}

impl<T> WeakRef<T>
where
    T: ?Sized,
{
    /// Create a new weak reference from another weak reference.
    pub fn new_with_uuid(value: Weak<RwLock<T>>, uuid: Uuid) -> WeakRef<T> {
        WeakRef { uuid, value }
    }

    /// Create a new weak reference from another strong reference.
    pub fn from_arc_with_uuid(value: Arc<RwLock<T>>, uuid: Uuid) -> WeakRef<T> {
        WeakRef {
            uuid,
            value: Arc::downgrade(&value),
        }
    }

    /// Create a new weak reference from a strong reference.
    pub fn from_smartref(strong: &SmartRef<T>) -> WeakRef<T> {
        WeakRef::new_with_uuid(Arc::downgrade(&strong.value), strong.uuid)
    }

    /// Attempt to upgrade the weak reference to a strong reference.
    pub fn get(&self) -> Option<SmartRef<T>> {
        self.value.upgrade().map(|arc| SmartRef::new_with_uuid(arc, self.uuid))
    }

    /// Get the number of references to the underlying smart pointer.
    pub fn strong_count(&self) -> usize {
        self.value.strong_count()
    }
}

impl<T> WeakRef<T>
where
    T: WithUuid,
{
    pub fn from_weak(value: Weak<RwLock<T>>) -> Result<WeakRef<T>> {
        let uuid = value
            .upgrade()
            .ok_or_else(|| anyhow::anyhow!("Could not read entity"))?
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .uuid();

        Ok(WeakRef::new_with_uuid(value, uuid))
    }

    pub fn from_arc(value: Arc<RwLock<T>>) -> Result<WeakRef<T>> {
        let uuid = value
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .map(|v| v.uuid())?;

        Ok(WeakRef::from_arc_with_uuid(value, uuid))
    }
}

impl<T> WithUuid for WeakRef<T> {
    /// Get the UUID of the stored entity. This is guaranteed to be a cheap operation.
    fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl<T> fmt::Debug for WeakRef<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SmartRef<{}> {{ {}, uses: {} }}",
            std::any::type_name::<T>(),
            self.uuid,
            self.value.strong_count()
        )
    }
}

impl<T> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        WeakRef {
            uuid: self.uuid,
            value: self.value.clone(),
        }
    }
}

impl<T> PartialEq for WeakRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> Ord for WeakRef<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<T> PartialOrd for WeakRef<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for WeakRef<T> {}

impl<T> Hash for WeakRef<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<T> From<SmartRef<T>> for WeakRef<T>
where
    T: ?Sized,
{
    fn from(item: SmartRef<T>) -> Self {
        WeakRef::from_smartref(&item)
    }
}
