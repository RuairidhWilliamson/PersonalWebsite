#![allow(clippy::unwrap_used)]

use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait MutexExt<T> {
    type Guard<'a>
    where
        Self: 'a;

    fn plock(&self) -> Self::Guard<'_>;
}

impl<T> MutexExt<T> for Mutex<T> {
    type Guard<'a>
        = MutexGuard<'a, T>
    where
        T: 'a;

    fn plock(&self) -> Self::Guard<'_> {
        self.lock().unwrap()
    }
}

pub trait RwLockExt<T> {
    type ReadGuard<'a>
    where
        Self: 'a;
    type WriteGuard<'a>
    where
        Self: 'a;

    fn pread(&self) -> Self::ReadGuard<'_>;
    fn pwrite(&self) -> Self::WriteGuard<'_>;
}

impl<T> RwLockExt<T> for RwLock<T> {
    type ReadGuard<'a>
        = RwLockReadGuard<'a, T>
    where
        T: 'a;

    type WriteGuard<'a>
        = RwLockWriteGuard<'a, T>
    where
        T: 'a;

    fn pread(&self) -> Self::ReadGuard<'_> {
        self.read().unwrap()
    }

    fn pwrite(&self) -> Self::WriteGuard<'_> {
        self.write().unwrap()
    }
}
