use std::sync::Mutex;
use std::collections::HashSet;

use crate::blob_id::BlobId;

/// A set of `BlobId` values, designed for concurrent modification.
///
/// This implementation imposes an equivalence relation on blob IDs, assigning each to one of 256
/// classes (based on its first byte). Each class is represented by a standard `HashMap` protected
/// by a `Mutex`. Since blob IDs are SHA-1 digests, and hence effectively random, the odds that two
/// random blob IDs appear in the same class is 1/256.
pub struct BlobIdSet {
    sets: [Mutex<HashSet<BlobId>>; 256],
}

impl BlobIdSet {
    pub fn new() -> Self {
        BlobIdSet {
            // What's this weird initialization?
            // It's to get around the fact that `Mutex` is not `Copy`.
            // https://stackoverflow.com/a/69756635
            sets: [(); 256].map(|_| Mutex::new(HashSet::with_capacity(1024))),
        }
    }

    /// Add the given `BlobId` to the set.
    /// The return value indicates whether the set was modified by this operation.
    #[inline]
    pub fn insert(&self, blob_id: BlobId) -> bool {
        let bucket: u8 = blob_id.bytes()[0];
        self.sets[bucket as usize].lock().unwrap().insert(blob_id)
    }

    /// Check if the given `BlobId` is in the set without modifying it.
    #[inline]
    pub fn contains(&self, blob_id: &BlobId) -> bool {
        let bucket: u8 = blob_id.bytes()[0];
        self.sets[bucket as usize].lock().unwrap().contains(blob_id)
    }

    /// Return the total number of blob IDs contained in the set.
    /// Note: this is not a cheap operation.
    pub fn len(&self) -> usize {
        self.sets.iter().map(|b| b.lock().unwrap().len()).sum()
    }
}
