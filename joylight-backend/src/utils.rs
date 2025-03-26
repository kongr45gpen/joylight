//! Utility definitions

use uuid::Uuid;
use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard, Weak};
use anyhow::Result;

pub trait WithUuid {
    fn uuid(&self) -> Uuid;
}

pub struct SmartRef<T> {
    pub uuid: Uuid,
    value: Arc<RwLock<T>>,
}

impl <T> SmartRef<T> {
    pub fn new_with_uuid(value: Arc<RwLock<T>>, uuid: Uuid) -> SmartRef<T> {
        // let value = Arc::downgrade(&value);

        SmartRef {
            uuid,
            value,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get(&self) -> Option<Arc<RwLock<T>>> {
        Some(self.value.clone())
    }

    pub fn read<F,R>(&self, f: F) -> Result<R> where F: Fn(&T) -> R {
        //TODO: Log errors
        let arc = Some(&self.value)
            .ok_or_else(|| anyhow::anyhow!("Attempted to read entity {} which has been removed", self.uuid))?;

        let guard = arc.read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(f(&guard))
    }

    pub fn write<F,R>(&self, f: F) -> Result<R> where F: Fn(&mut T) -> R {
        let arc = Some(&self.value)
            .ok_or_else(|| anyhow::anyhow!("Attempted to edit entity {} which has been removed", self.uuid))?;

        let mut guard = arc.write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(f(&mut guard))
    }
}

impl <T> SmartRef<T> where T: WithUuid {
    pub fn new(value: Arc<RwLock<T>>) -> Result<SmartRef<T>> {
        let uuid = value.read()
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

impl <T> fmt::Debug for SmartRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmartRef<{}> {{ uuid: {} }}", std::any::type_name::<T>(), self.uuid)
    }
}

impl <T> Clone for SmartRef<T> {
    fn clone(&self) -> Self {
        SmartRef {
            uuid: self.uuid,
            value: self.value.clone(),
        }
    }
}