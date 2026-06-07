//! A hash map where the iteration order is based on insertion order.
//!
//! It wraps over a [`HashMap`]() for key->sequence id lookups,
//! and an [`OrdMap`] for mapping incrementing sequence ids->values and tracking insertion order.
//!
//! It's an imbl-based implementation of [`indexmap`](https://docs.rs/indexmap)'s [`IndexMap`],
//! with the notable exception of direct-index access such as [`get_index(usize)`].
//! Providing index-based access would require a `Vector<V>` value and `HashMap<K, usize>` implementation,
//! which would have crippled `shift_remove()` performance, or a smarter/more complex data structure.
//!
//! If [OrdMap is ever updated to support index access](https://github.com/jneem/imbl/issues/158) by tracking
//! subtree sizes, this data structure would get indexed access for free. Until then, `.iter().nth()` must be used.
//!
//! Iterating over this map will provide properties in insertion order; updating a key does not change its position.
//! Removing a key and re-inserting it will.
//!
//! [`HashMap`]: https://docs.rs/imbl/latest/imbl/type.HashMap.html
//! [`OrdMap`]: https://docs.rs/imbl/latest/imbl/type.OrdMap.html
//! [`IndexMap`]: https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html
//! [`get_index(usize)`]: https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html#method.get_index

use std::borrow::Borrow;
use std::collections;
use std::collections::hash_map::RandomState;
use std::fmt::{Debug, Error, Formatter};
use std::hash::{BuildHasher, Hash};
use std::iter::{FromIterator, FusedIterator, Sum};
use std::ops::Add;
use std::ops::Index;

use imbl::GenericHashMap;
use imbl::ordmap::{self, GenericOrdMap};
use imbl::shared_ptr::{DefaultSharedPtr, SharedPointerKind};

/// Construct an index map from a sequence of key/value pairs.
///
/// # Examples
///
/// ```
/// use imbl_index::indexmap;
/// assert_eq!(
///   indexmap!{
///     1 => 11,
///     2 => 22,
///     3 => 33
///   },
///   imbl_index::IndexMap::from(vec![(1, 11), (2, 22), (3, 33)])
/// );
/// ```
#[macro_export]
macro_rules! indexmap {
    () => { $crate::IndexMap::new() };

    ( $( $key:expr => $value:expr ),* ) => {{
        let mut map = $crate::IndexMap::new();
        $({
            map.insert($key, $value);
        })*;
        map
    }};

    ( $( $key:expr => $value:expr ,)* ) => {{
        let mut map = $crate::IndexMap::new();
        $({
            map.insert($key, $value);
        })*;
        map
    }};
}

/// Type alias for [`GenericIndexMap`] that uses
/// [`std::hash::RandomState`] as the default hasher and
/// [`DefaultSharedPtr`] as the pointer type.
pub type IndexMap<K, V> = GenericIndexMap<K, V, RandomState, DefaultSharedPtr>;

/// A hash map where the iteration order is based on insertion order.
///
/// Most operations on this map are O(log n). Due to the extra indirection,
/// it will be both slower than an OrdMap and a HashMap.
pub struct GenericIndexMap<K, V, S, P: SharedPointerKind> {
    index: GenericHashMap<K, u64, S, P>,
    order: GenericOrdMap<u64, (K, V), P>,
    next_index: u64,
}

impl<K, V, S, P: SharedPointerKind> GenericIndexMap<K, V, S, P> {
    /// Construct an empty index map.
    #[inline]
    #[must_use]
    pub fn new() -> Self
    where
        S: Default,
    {
        Self {
            index: GenericHashMap::with_hasher(Default::default()),
            order: GenericOrdMap::new(),
            next_index: 0,
        }
    }

    /// Construct an empty index map using the provided hasher.
    #[inline]
    #[must_use]
    pub fn with_hasher(hasher: S) -> Self {
        GenericIndexMap {
            index: GenericHashMap::with_hasher(hasher),
            order: GenericOrdMap::new(),
            next_index: 0,
        }
    }

    /// Get the size of an index map.
    ///
    /// Time: O(1)
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Test whether an index map is empty.
    ///
    /// Time: O(1)
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Test whether two maps refer to the same content in memory.
    ///
    /// This is true if the two sides are references to the same map,
    /// or if the two maps refer to the same root node.
    ///
    /// Time: O(1)
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.index.ptr_eq(&other.index)
    }

    /// Get a reference to the map's [`BuildHasher`].
    #[must_use]
    pub fn hasher(&self) -> &S {
        self.index.hasher()
    }

    /// Discard all elements from the map.
    ///
    /// This leaves you with an empty map, and all elements that
    /// were previously inside it are dropped.
    ///
    /// Time: O(n)
    pub fn clear(&mut self)
    where
        S: Clone,
    {
        self.index = GenericHashMap::with_hasher(self.index.hasher().clone());
        self.order = GenericOrdMap::new();
        self.next_index = 0;
    }

    /// Get an iterator over the key/value pairs of an index map.
    ///
    /// The items will be returned in insertion order.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V, P> {
        Iter {
            it: self.order.iter(),
        }
    }

    /// Get an iterator over an index map's keys, in insertion order.
    #[inline]
    #[must_use]
    pub fn keys(&self) -> Keys<'_, K, V, P> {
        Keys {
            it: self.order.iter(),
        }
    }

    /// Get an iterator over an index map's values, in insertion order.
    #[inline]
    #[must_use]
    pub fn values(&self) -> Values<'_, K, V, P> {
        Values {
            it: self.order.iter(),
        }
    }

    /// Get an iterator over the differences between this map and
    /// another, i.e. the set of entries to add, update, or remove to
    /// this map in order to make it equal to the other map.
    ///
    /// This function will avoid visiting nodes which are shared
    /// between the two sets, meaning that even very large sets can be
    /// compared quickly if most of their structure is shared.
    ///
    /// Time: O(n) where n is the size of the larger map.
    #[must_use]
    pub fn diff<'a, 'b>(&'a self, other: &'b Self) -> ordmap::DiffIter<'a, 'b, u64, (K, V), P>
    where
        K: Ord,
        V: PartialEq,
        P: SharedPointerKind,
    {
        self.order.diff(&other.order)
    }

    /// Get the first entry in insertion order, if any.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn front(&self) -> Option<(&K, &V)> {
        self.order.get_min().map(|(_, (k, v))| (k, v))
    }

    /// Get the last entry in insertion order, if any.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn back(&self) -> Option<(&K, &V)> {
        self.order.get_max().map(|(_, (k, v))| (k, v))
    }

    /// Get the entry at a given index in insertion order.
    ///
    /// Time: O(n)
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.iter().nth(index)
    }
}

impl<K, V, S, P> GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq,
    S: BuildHasher + Clone,
    P: SharedPointerKind,
{
    /// Get the value for a key from an index map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        let seq = self.index.get(key)?;
        self.order.get(seq).map(|(_, v)| v)
    }

    /// Get the key/value pair for a key from an index map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        let seq = self.index.get(key)?;
        self.order.get(seq).map(|(k, v)| (k, v))
    }

    /// Test for the presence of a key in an index map.
    ///
    /// Time: O(log n)
    #[inline]
    #[must_use]
    pub fn contains_key(&self, k: &K) -> bool {
        self.get(k).is_some()
    }

    /// Test whether a map is a submap of another map, meaning that
    /// all keys in our map must also be in the other map, with the
    /// same values.
    ///
    /// Use the provided function to decide whether values are equal.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn is_submap_by<B, RM, F, P2: SharedPointerKind>(&self, other: RM, mut cmp: F) -> bool
    where
        F: FnMut(&V, &B) -> bool,
        RM: Borrow<GenericIndexMap<K, B, S, P2>>,
    {
        self.iter()
            .all(|(k, v)| other.borrow().get(k).map(|ov| cmp(v, ov)).unwrap_or(false))
    }

    /// Test whether a map is a proper submap of another map, meaning
    /// that all keys in our map must also be in the other map, with
    /// the same values. To be a proper submap, ours must also contain
    /// fewer keys than the other map.
    ///
    /// Use the provided function to decide whether values are equal.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn is_proper_submap_by<B, RM, F, P2: SharedPointerKind>(&self, other: RM, cmp: F) -> bool
    where
        F: FnMut(&V, &B) -> bool,
        RM: Borrow<GenericIndexMap<K, B, S, P2>>,
    {
        self.len() != other.borrow().len() && self.is_submap_by(other, cmp)
    }

    /// Test whether a map is a submap of another map, meaning that
    /// all keys in our map must also be in the other map, with the
    /// same values.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn is_submap<RM>(&self, other: RM) -> bool
    where
        V: PartialEq,
        RM: Borrow<Self>,
    {
        self.is_submap_by(other.borrow(), PartialEq::eq)
    }

    /// Test whether a map is a proper submap of another map, meaning
    /// that all keys in our map must also be in the other map, with
    /// the same values. To be a proper submap, ours must also contain
    /// fewer keys than the other map.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn is_proper_submap<RM>(&self, other: RM) -> bool
    where
        V: PartialEq,
        RM: Borrow<Self>,
    {
        self.is_proper_submap_by(other.borrow(), PartialEq::eq)
    }
}

impl<K, V, S, P> GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone,
    P: SharedPointerKind,
{
    /// Insert a key/value mapping into an index map.
    ///
    /// If the map already has a mapping for the given key, the
    /// previous value is overwritten and returned. The insertion
    /// order of the key is not changed.
    ///
    /// Time: O(log n)
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old_seq = self.index.get(&k).copied();
        match old_seq {
            Some(seq) => self.order.insert(seq, (k, v)).map(|(_, old_v)| old_v),
            None => {
                let seq = self.next_index;
                self.next_index += 1;
                self.order.insert(seq, (k.clone(), v.clone()));
                self.index.insert(k, seq);
                None
            }
        }
    }

    /// Remove a key/value pair from an index map, if it exists.
    ///
    /// Time: O(log n)
    pub fn remove(&mut self, k: &K) -> Option<V> {
        let seq = self.index.remove(k)?;
        let (_, v) = self.order.remove(&seq)?;
        Some(v)
    }

    /// Construct a new index map by inserting a key/value mapping
    /// into a map.
    ///
    /// If the map already has a mapping for the given key, the
    /// previous value is overwritten. The insertion order of the
    /// key is not changed.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn update(&self, k: K, v: V) -> Self {
        let mut out = self.clone();
        out.insert(k, v);
        out
    }

    /// Construct a new map by inserting a key/value mapping into
    /// a map.
    ///
    /// If the map already has a mapping for the given key, we call
    /// the provided function with the old value and the new value,
    /// and insert the result as the new value.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn update_with<F>(self, k: K, v: V, f: F) -> Self
    where
        F: FnOnce(V, V) -> V,
    {
        self.update_with_key(k, v, |_, v1, v2| f(v1, v2))
    }

    /// Construct a new map by inserting a key/value mapping into
    /// a map.
    ///
    /// If the map already has a mapping for the given key, we call
    /// the provided function with the key, the old value and the new
    /// value, and insert the result as the new value.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn update_with_key<F>(self, k: K, v: V, f: F) -> Self
    where
        F: FnOnce(&K, V, V) -> V,
    {
        match self.extract_with_key(&k) {
            None => self.update(k, v),
            Some((_, v2, m)) => {
                let out_v = f(&k, v2, v);
                m.update(k, out_v)
            }
        }
    }

    /// Construct a new map by inserting a key/value mapping into a
    /// map, returning the old value for the key as well as the new
    /// map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn update_lookup_with_key<F>(self, k: K, v: V, f: F) -> (Option<V>, Self)
    where
        F: FnOnce(&K, &V, V) -> V,
    {
        match self.extract_with_key(&k) {
            None => (None, self.update(k, v)),
            Some((_, v2, m)) => {
                let out_v = f(&k, &v2, v);
                (Some(v2), m.update(k, out_v))
            }
        }
    }

    /// Update the value for a given key by calling a function with
    /// the current value and overwriting it with the function's
    /// return value.
    ///
    /// The function gets an [`Option<V>`] and
    /// returns the same, so that it can decide to delete a mapping
    /// instead of updating the value, and decide what to do if the
    /// key isn't in the map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn alter<F>(&self, f: F, k: K) -> Self
    where
        F: FnOnce(Option<V>) -> Option<V>,
    {
        let old_v = self.get(&k).cloned();
        match f(old_v) {
            None if self.contains_key(&k) => self.without(&k),
            None => self.clone(),
            Some(v) => self.update(k, v),
        }
    }

    /// Remove a key/value pair from a map, if it exists, and return
    /// the updated map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn without(&self, k: &K) -> Self {
        self.extract(k)
            .map(|(_, m)| m)
            .unwrap_or_else(|| self.clone())
    }

    /// Remove a key/value pair from a map, if it exists, and return
    /// the removed value as well as the updated map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn extract(&self, k: &K) -> Option<(V, Self)> {
        self.extract_with_key(k).map(|(_, v, m)| (v, m))
    }

    /// Remove a key/value pair from a map, if it exists, and return
    /// the removed key and value as well as the updated map.
    ///
    /// Time: O(log n)
    #[must_use]
    pub fn extract_with_key(&self, k: &K) -> Option<(K, V, Self)> {
        let mut out = self.clone();
        let seq = out.index.remove(k)?;
        let (old_k, old_v) = out.order.remove(&seq)?;
        Some((old_k, old_v, out))
    }

    /// Construct the union of two maps, keeping the values in the
    /// current map when keys exist in both maps.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn union(mut self, mut other: Self) -> Self {
        if self.len() >= other.len() {
            for (k, v) in other {
                self.entry(k).or_insert(v);
            }
            self
        } else {
            for (k, v) in self {
                other.entry(k).or_insert(v);
            }
            other
        }
    }

    /// Construct the union of two maps, using a function to decide
    /// what to do with the value when a key is in both maps.
    ///
    /// The function is called when a value exists in both maps, and
    /// receives the value from the current map as its first argument,
    /// and the value from the other map as the second. It should
    /// return the value to be inserted in the resulting map.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn union_with<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(V, V) -> V,
    {
        self.union_with_key(other, |_, v1, v2| f(v1, v2))
    }

    /// Construct the union of two maps, using a function to decide
    /// what to do with the value when a key is in both maps.
    ///
    /// The function is called when a value exists in both maps, and
    /// receives a reference to the key as its first argument, the
    /// value from the current map as the second argument, and the
    /// value from the other map as the third argument. It should
    /// return the value to be inserted in the resulting map.
    ///
    /// Time: O(n log n)
    #[must_use]
    pub fn union_with_key<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(&K, V, V) -> V,
    {
        if self.len() >= other.len() {
            self.union_with_key_inner(other, f)
        } else {
            other.union_with_key_inner(self, |key, other_value, self_value| {
                f(key, self_value, other_value)
            })
        }
    }

    fn union_with_key_inner<F>(mut self, other: Self, mut f: F) -> Self
    where
        F: FnMut(&K, V, V) -> V,
    {
        for (k, v) in other {
            let old = self.get(&k).cloned();
            let v = match old {
                Some(old_v) => f(&k, old_v, v),
                None => v,
            };
            self.insert(k, v);
        }
        self
    }

    /// Get the [`Entry`] for a key in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S, P> {
        if self.index.contains_key(&key) {
            Entry::Occupied(OccupiedEntry { map: self, key })
        } else {
            Entry::Vacant(VacantEntry { map: self, key })
        }
    }
}

// Core traits

impl<K, V, S, P> Clone for GenericIndexMap<K, V, S, P>
where
    K: Clone,
    V: Clone,
    S: Clone,
    P: SharedPointerKind,
{
    /// Clone a map.
    ///
    /// Time: O(1)
    #[inline]
    fn clone(&self) -> Self {
        GenericIndexMap {
            index: self.index.clone(),
            order: self.order.clone(),
            next_index: self.next_index,
        }
    }
}

impl<K, V, S, P> Default for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq,
    V: Clone,
    S: BuildHasher + Default + Clone,
    P: SharedPointerKind,
{
    fn default() -> Self {
        Self {
            index: GenericHashMap::with_hasher(Default::default()),
            order: GenericOrdMap::new(),
            next_index: 0,
        }
    }
}

impl<K, V, S1, S2, P1, P2> PartialEq<GenericIndexMap<K, V, S2, P2>>
    for GenericIndexMap<K, V, S1, P1>
where
    K: PartialEq,
    V: PartialEq,
    P1: SharedPointerKind,
    P2: SharedPointerKind,
{
    fn eq(&self, other: &GenericIndexMap<K, V, S2, P2>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter()
            .zip(other.iter())
            .all(|((k1, v1), (k2, v2))| k1 == k2 && v1 == v2)
    }
}

impl<K: Eq + Hash, V: Eq, S: BuildHasher + Clone, P: SharedPointerKind> Eq
    for GenericIndexMap<K, V, S, P>
{
}

impl<K, V, S, P> Debug for GenericIndexMap<K, V, S, P>
where
    K: Debug,
    V: Debug,
    P: SharedPointerKind,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut d = f.debug_map();
        for (k, v) in self.iter() {
            d.entry(k, v);
        }
        d.finish()
    }
}

impl<K, V, S, P: SharedPointerKind> Add for &GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone + Default,
    P: SharedPointerKind,
{
    type Output = GenericIndexMap<K, V, S, P>;

    fn add(self, other: Self) -> Self::Output {
        self.clone().union(other.clone())
    }
}

impl<K, V, S, P: SharedPointerKind> Add for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone + Default,
    P: SharedPointerKind,
{
    type Output = GenericIndexMap<K, V, S, P>;

    fn add(self, other: Self) -> Self::Output {
        self.union(other)
    }
}

impl<K, V, S, P: SharedPointerKind> Sum for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone + Default,
    P: SharedPointerKind,
{
    fn sum<I>(it: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        it.fold(Self::default(), |a, b| a + b)
    }
}

// Index trait

impl<K, V, S, P> Index<&K> for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq,
    S: BuildHasher + Clone,
    P: SharedPointerKind,
{
    type Output = V;

    fn index(&self, key: &K) -> &V {
        self.get(key)
            .expect("GenericIndexMap::index: key not found")
    }
}

// Conversions

impl<K, V, P: SharedPointerKind> AsRef<GenericIndexMap<K, V, RandomState, P>>
    for GenericIndexMap<K, V, RandomState, P>
{
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'a, K, V, RK, RV, OK, OV, S, P> From<&'a [(RK, RV)]> for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone + From<OK>,
    V: Clone + From<OV>,
    OV: Borrow<RV>,
    RK: ToOwned<Owned = OK>,
    RV: ToOwned<Owned = OV>,
    S: BuildHasher + Default + Clone,
    P: SharedPointerKind,
{
    fn from(m: &'a [(RK, RV)]) -> GenericIndexMap<K, V, S, P> {
        m.iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }
}

impl<K, V, RK, RV, S, P> From<Vec<(RK, RV)>> for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone + From<RK>,
    V: Clone + From<RV>,
    S: BuildHasher + Default + Clone,
    P: SharedPointerKind,
{
    fn from(m: Vec<(RK, RV)>) -> GenericIndexMap<K, V, S, P> {
        m.into_iter().collect()
    }
}

impl<'a, K, V, RK, RV, OK, OV, S, P> From<&'a Vec<(RK, RV)>> for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone + From<OK>,
    V: Clone + From<OV>,
    OV: Borrow<RV>,
    RK: ToOwned<Owned = OK>,
    RV: ToOwned<Owned = OV>,
    S: BuildHasher + Default + Clone,
    P: SharedPointerKind,
{
    fn from(m: &'a Vec<(RK, RV)>) -> GenericIndexMap<K, V, S, P> {
        m.iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }
}

impl<K, V, RK, RV, S, P> From<collections::HashMap<RK, RV>> for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone + From<RK>,
    V: Clone + From<RV>,
    RK: Hash + Eq,
    S: BuildHasher + Default + Clone,
    P: SharedPointerKind,
{
    fn from(m: collections::HashMap<RK, RV>) -> GenericIndexMap<K, V, S, P> {
        m.into_iter().collect()
    }
}

impl<K, V, S1, P1, P2> From<GenericHashMap<K, V, S1, P2>> for GenericIndexMap<K, V, S1, P1>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S1: BuildHasher + Clone + Default,
    P1: SharedPointerKind,
    P2: SharedPointerKind,
{
    fn from(m: GenericHashMap<K, V, S1, P2>) -> Self {
        m.into_iter().collect()
    }
}

// FromIterator

impl<K, V, RK, RV, S, P> FromIterator<(RK, RV)> for GenericIndexMap<K, V, S, P>
where
    K: Hash + Eq + Clone + From<RK>,
    V: Clone + From<RV>,
    S: BuildHasher + Default + Clone,
    P: SharedPointerKind,
{
    fn from_iter<T>(i: T) -> Self
    where
        T: IntoIterator<Item = (RK, RV)>,
    {
        let mut m = Self::default();
        for (k, v) in i {
            m.insert(From::from(k), From::from(v));
        }
        m
    }
}

impl<'a, K, V, S, P> IntoIterator for &'a GenericIndexMap<K, V, S, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V, S, P> IntoIterator for GenericIndexMap<K, V, S, P>
where
    K: Clone,
    V: Clone,
    P: SharedPointerKind,
{
    type Item = (K, V);
    type IntoIter = ConsumingIter<K, V, P>;

    fn into_iter(self) -> Self::IntoIter {
        ConsumingIter {
            it: self.order.into_iter(),
        }
    }
}

// Entry API

/// A view into a single entry in a map, which may either be vacant or occupied.
pub enum Entry<'a, K, V, S, P: SharedPointerKind> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, S, P>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, S, P>),
}

impl<'a, K, V, S, P> Entry<'a, K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone,
    P: SharedPointerKind,
{
    /// Get the key for this entry.
    #[must_use]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(e) => e.key(),
            Entry::Vacant(e) => e.key(),
        }
    }

    /// Return a mutable reference to this entry if it's occupied,
    /// or `None` if it's vacant.
    pub fn as_mut(&mut self) -> Option<&mut V> {
        match self {
            Entry::Occupied(e) => Some(e.get_mut()),
            Entry::Vacant(_) => None,
        }
    }

    /// Return the value if this entry is occupied, along with a
    /// mutable reference to the map, allowing in-place manipulation.
    pub fn as_ref(&self) -> Option<&V> {
        match self {
            Entry::Occupied(e) => Some(e.get()),
            Entry::Vacant(_) => None,
        }
    }

    /// Insert a value into this entry if it's vacant, and return
    /// a mutable reference to the value.
    pub fn or_insert(self, value: V) -> &'a mut V {
        self.or_insert_with(|| value)
    }

    /// Insert a value into this entry if it's vacant, using the
    /// provided function to generate the value, and return a mutable
    /// reference to the value.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default()),
        }
    }

    /// Insert a value into this entry if it's vacant, using the
    /// provided function to generate the value from the key, and
    /// return a mutable reference to the value.
    pub fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let v = default(e.key());
                e.insert(v)
            }
        }
    }

    /// Update this entry by mutating its value if occupied, or
    /// inserting a new value if vacant.
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Entry::Occupied(ref mut e) = self {
            f(e.get_mut());
        }
        self
    }
}

/// An occupied entry.
pub struct OccupiedEntry<'a, K, V, S, P: SharedPointerKind> {
    map: &'a mut GenericIndexMap<K, V, S, P>,
    key: K,
}

impl<'a, K, V, S, P> OccupiedEntry<'a, K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone,
    P: SharedPointerKind,
{
    /// Get the key for this entry.
    #[must_use]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Remove this entry from the map and return the removed mapping.
    pub fn remove_entry(self) -> (K, V) {
        let seq = self
            .map
            .index
            .remove(&self.key)
            .expect("GenericIndexMap::OccupiedEntry::remove_entry: key has vanished!");
        let (k, v) = self
            .map
            .order
            .remove(&seq)
            .expect("GenericIndexMap::OccupiedEntry::remove_entry: seq has vanished!");
        (k, v)
    }

    /// Get the current value.
    #[must_use]
    pub fn get(&self) -> &V {
        let seq = self.map.index.get(&self.key).unwrap();
        let (_, v) = self.map.order.get(seq).unwrap();
        v
    }

    /// Get a mutable reference to the current value.
    #[must_use]
    pub fn get_mut(&mut self) -> &mut V {
        let seq = self.map.index.get(&self.key).copied().unwrap();
        let (_, v) = self.map.order.get_mut(&seq).unwrap();
        v
    }

    /// Convert this entry into a mutable reference.
    #[must_use]
    pub fn into_mut(self) -> &'a mut V {
        let seq = self.map.index.get(&self.key).copied().unwrap();
        let (_, v) = self.map.order.get_mut(&seq).unwrap();
        v
    }

    /// Overwrite the current value.
    pub fn insert(&mut self, value: V) -> V {
        use std::mem;
        mem::replace(self.get_mut(), value)
    }

    /// Remove this entry from the map and return the removed value.
    pub fn remove(self) -> V {
        self.remove_entry().1
    }
}

/// A vacant entry.
pub struct VacantEntry<'a, K, V, S, P: SharedPointerKind> {
    map: &'a mut GenericIndexMap<K, V, S, P>,
    key: K,
}

impl<'a, K, V, S, P> VacantEntry<'a, K, V, S, P>
where
    K: Hash + Eq + Clone,
    V: Clone,
    S: BuildHasher + Clone,
    P: SharedPointerKind,
{
    /// Get the key for this entry.
    #[must_use]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Convert this entry into its key.
    #[must_use]
    pub fn into_key(self) -> K {
        self.key
    }

    /// Insert a value into this entry.
    pub fn insert(self, value: V) -> &'a mut V {
        let seq = self.map.next_index;
        self.map.next_index += 1;
        self.map
            .order
            .insert(seq, (self.key.clone(), value.clone()));
        self.map.index.insert(self.key, seq);
        self.map.order.get_mut(&seq).map(|(_, v)| v).unwrap()
    }
}

// Iterators

/// An iterator over the key/value pairs of an index map.
pub struct Iter<'a, K, V, P: SharedPointerKind> {
    it: ordmap::Iter<'a, u64, (K, V), P>,
}

impl<'a, K, V, P: SharedPointerKind> Clone for Iter<'a, K, V, P> {
    fn clone(&self) -> Self {
        Iter {
            it: self.it.clone(),
        }
    }
}

impl<'a, K, V, P> Iterator for Iter<'a, K, V, P>
where
    P: SharedPointerKind,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|(_, (k, v))| (k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}

impl<'a, K, V, P> DoubleEndedIterator for Iter<'a, K, V, P>
where
    P: SharedPointerKind,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|(_, (k, v))| (k, v))
    }
}

impl<'a, K, V, P> ExactSizeIterator for Iter<'a, K, V, P> where P: SharedPointerKind {}

impl<'a, K, V, P> FusedIterator for Iter<'a, K, V, P> where P: SharedPointerKind {}

/// An iterator over the keys of an index map.
pub struct Keys<'a, K, V, P: SharedPointerKind> {
    it: ordmap::Iter<'a, u64, (K, V), P>,
}

impl<'a, K, V, P> Iterator for Keys<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|(_, (k, _))| k)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}

impl<'a, K, V, P> DoubleEndedIterator for Keys<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|(_, (k, _))| k)
    }
}

impl<'a, K, V, P> ExactSizeIterator for Keys<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
}

impl<'a, K, V, P> FusedIterator for Keys<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
}

/// An iterator over the values of an index map.
pub struct Values<'a, K, V, P: SharedPointerKind> {
    it: ordmap::Iter<'a, u64, (K, V), P>,
}

impl<'a, K, V, P> Iterator for Values<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|(_, (_, v))| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}

impl<'a, K, V, P> DoubleEndedIterator for Values<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|(_, (_, v))| v)
    }
}

impl<'a, K, V, P> ExactSizeIterator for Values<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
}

impl<'a, K, V, P> FusedIterator for Values<'a, K, V, P>
where
    K: 'a,
    V: 'a,
    P: SharedPointerKind,
{
}

/// A consuming iterator over the elements of an index map.
pub struct ConsumingIter<K, V, P: SharedPointerKind> {
    it: ordmap::ConsumingIter<u64, (K, V), P>,
}

impl<K, V, P> Iterator for ConsumingIter<K, V, P>
where
    K: Clone,
    V: Clone,
    P: SharedPointerKind,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|(_, kv)| kv)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}

impl<K: Clone, V: Clone, P: SharedPointerKind> DoubleEndedIterator for ConsumingIter<K, V, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|(_, kv)| kv)
    }
}

impl<K, V, P> ExactSizeIterator for ConsumingIter<K, V, P>
where
    K: Clone,
    V: Clone,
    P: SharedPointerKind,
{
}

impl<K, V, P> FusedIterator for ConsumingIter<K, V, P>
where
    K: Clone,
    V: Clone,
    P: SharedPointerKind,
{
}

// Tests

#[cfg(test)]
mod test {
    use super::*;
    use ::proptest::collection;
    use ::proptest::num::i16;
    use ::proptest::prelude::*;
    use std::collections::HashMap as StdHashMap;

    #[test]
    fn iterates_in_insertion_order() {
        let map = indexmap! {
            2 => 22,
            1 => 11,
            3 => 33,
        };
        let mut it = map.iter();
        assert_eq!(it.next(), Some((&2, &22)));
        assert_eq!(it.next(), Some((&1, &11)));
        assert_eq!(it.next(), Some((&3, &33)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn reinsert_doesnt_change_order() {
        let mut map = indexmap! {
            "a" => 1,
            "b" => 2,
            "c" => 3,
        };
        map.insert("b", 42);
        let mut it = map.iter();
        assert_eq!(it.next(), Some((&"a", &1)));
        assert_eq!(it.next(), Some((&"b", &42)));
        assert_eq!(it.next(), Some((&"c", &3)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn remove_and_reinsert_moves_to_end() {
        let mut map = indexmap! {
            "a" => 1,
            "b" => 2,
            "c" => 3,
        };
        map.remove(&"a");
        map.insert("a", 99);
        let mut it = map.iter();
        assert_eq!(it.next(), Some((&"b", &2)));
        assert_eq!(it.next(), Some((&"c", &3)));
        assert_eq!(it.next(), Some((&"a", &99)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn front_and_back() {
        let map = indexmap! { 1 => 11, 2 => 22, 3 => 33 };
        assert_eq!(map.front(), Some((&1, &11)));
        assert_eq!(map.back(), Some((&3, &33)));
    }

    #[test]
    fn empty_map() {
        let map = IndexMap::<i32, i32>::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.front(), None);
        assert_eq!(map.back(), None);
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn from_vec() {
        let map: IndexMap<i32, i32> = IndexMap::from(vec![(1, 11), (2, 22), (3, 33)]);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&2), Some(&22));
    }

    #[test]
    fn alter_preserves_order() {
        let mut map = indexmap! {
            "one" => 1,
            "two" => 2,
            "three" => 3
        };

        map = map.alter(|n| n.map(|n| n * 2), "two");
        map = map.alter(|_| Some(4), "four");
        map = map.alter(|_| None, "five");

        assert_eq!(
            map.into_iter().collect::<Vec<_>>(),
            vec![("one", 1), ("two", 4), ("three", 3), ("four", 4),]
        );
    }

    proptest! {
        #[test]
        fn prop_insert_and_get(
            ref pairs in collection::vec((i16::ANY, i16::ANY), 0..100),
        ) {
            let mut map = IndexMap::new();
            let mut std_map = StdHashMap::new();
            for &(k, v) in pairs {
                map.insert(k, v);
                std_map.insert(k, v);
            }
            for (k, v) in &std_map {
                assert_eq!(map.get(k), Some(v));
            }
            assert_eq!(map.len(), std_map.len());
        }

        #[test]
        fn prop_remove(
            ref pairs in collection::vec((i16::ANY, i16::ANY), 0..100),
            ref to_remove in collection::vec(i16::ANY, 0..50),
        ) {
            let mut map = IndexMap::new();
            let mut std_map = StdHashMap::new();
            for &(k, v) in pairs {
                map.insert(k, v);
                std_map.insert(k, v);
            }
            for k in to_remove {
                let map_v = map.remove(k);
                let std_v = std_map.remove(k);
                assert_eq!(map_v, std_v);
            }
            assert_eq!(map.len(), std_map.len());
            for (k, v) in &std_map {
                assert_eq!(map.get(k), Some(v));
            }
        }

        #[test]
        fn prop_insertion_order(
            ref pairs in collection::vec((i16::ANY, i16::ANY), 0..100),
        ) {
            let mut map = IndexMap::new();
            for &(k, v) in pairs {
                map.insert(k, v);
            }
            let mut seen = StdHashMap::new();
            for (k, v) in map.iter() {
                if let Some(&first_v) = seen.get(k) {
                    assert_eq!(v, &first_v);
                } else {
                    seen.insert(*k, *v);
                }
            }
        }
    }
}
