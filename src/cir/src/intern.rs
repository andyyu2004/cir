use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::lazy::SyncOnceCell;
use std::ops::Deref;
use std::sync::Arc;

use dashmap::{DashSet, SharedValue};
use rustc_hash::FxHasher;

use crate::TyData;

#[derive(Debug, Clone)]
pub struct Interned<T>(Arc<T>);

impl<T: Intern> Interned<T> {
    pub fn intern(x: T) -> Self {
        // FIXME This function causes an ICE quite often.
        // Downgrading to dashmap 4 (from 5) seems to avoid it?
        // Needs some more investigation
        let map = T::interner().get();
        let shard_idx = map.determine_map(&x);
        let mut shard = map.shards()[shard_idx].write();
        match shard.get_key_value(&x) {
            Some((interned, _)) => Self(Arc::clone(&interned)),
            None => {
                let arc = Arc::new(x);
                shard.insert(Arc::clone(&arc), SharedValue::new(()));
                Self(arc)
            }
        }
    }
}

impl<T: Intern> PartialEq for Interned<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Deref for Interned<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Intern> Eq for Interned<T> {
}

impl<T: Intern> Hash for Interned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(Arc::as_ptr(&self.0) as *const () as usize)
    }
}

type InternMap<T> = DashSet<Arc<T>, BuildHasherDefault<FxHasher>>;

pub struct Interner<T> {
    map: SyncOnceCell<InternMap<T>>,
}

impl<T: Intern> Interner<T> {
    fn get(&self) -> &InternMap<T> {
        self.map.get_or_init(Default::default)
    }
}

impl<T> Interner<T> {
    const fn new() -> Self {
        Self { map: SyncOnceCell::new() }
    }
}

pub trait Intern: Hash + Eq + Sized + 'static {
    fn interner() -> &'static Interner<Self>;

    fn intern(self) -> Interned<Self> {
        Interned::intern(self)
    }
}

impl Intern for TyData {
    fn interner() -> &'static Interner<Self> {
        static INTERNER: Interner<TyData> = Interner::new();
        &INTERNER
    }
}
