use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use imbl::hashmap::HashMap as ImHashMap;
use imbl::ordmap::OrdMap as ImOrdMap;
use indexmap::IndexMap as CrateIndexMap;
use std::collections::HashMap as StdHashMap;
use std::hash::Hash;
use std::hint::black_box;

use imbl_index::GenericIndexMap;
use imbl_index::IndexMap;

// ── Test data generation ─────────────────────────────────────────

trait TestData: Clone + Hash + Eq + std::fmt::Debug + 'static {
    fn generate(n: usize) -> Vec<Self> {
        use rand::prelude::*;
        let mut rng = SmallRng::seed_from_u64(42);

        let mut v: Vec<_> = (0..n).map(Self::generate_one).collect();

        v.shuffle(&mut rng);

        v
    }

    fn generate_one(i: usize) -> Self;
}

impl TestData for i64 {
    fn generate_one(i: usize) -> Self {
        i as i64
    }
}

impl TestData for String {
    fn generate_one(i: usize) -> Self {
        format!("key_{:016x}", i)
    }
}

// ── BigType: 4 KB newtype ──────────────────────────────────────────

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct BigType([u8; 4096]);

impl TestData for BigType {
    fn generate_one(i: usize) -> Self {
        let mut buf = [0; 4096];
        let i = i % 4096;
        let v = (i / 4096) as u8;

        buf[i] = v;

        BigType(buf)
    }
}

fn reorder<K: Clone>(keys: &[K]) -> Vec<K> {
    use rand::prelude::*;
    let mut rng = SmallRng::seed_from_u64(1);
    let mut order = keys.to_vec();
    order.shuffle(&mut rng);
    order
}

fn reorder_in_place<V>(values: &mut [V], rng_seed: u64) {
    use rand::prelude::*;
    let mut rng = SmallRng::seed_from_u64(rng_seed);
    values.shuffle(&mut rng);
}

// ── Benchmark trait ──────────────────────────────────────────────

trait BenchMap<K, V>: Clone + FromIterator<(K, V)>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    type Iter<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    fn new() -> Self;
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn insert_clone(&self, k: K, v: V) -> Self;
    fn remove(&mut self, k: &K) -> Option<V>;
    fn remove_clone(&self, k: &K) -> Self;
    fn get(&self, k: &K) -> Option<&V>;
    fn iter(&self) -> Self::Iter<'_>;

    fn has_expensive_clone() -> bool {
        false
    }

    fn has_expensive_remove() -> bool {
        false
    }

    fn keys_cloned<'a>(&'a self) -> impl Iterator<Item = K>
    where
        K: 'a,
        V: 'a,
    {
        self.iter().map(|(k, _)| k.clone())
    }
}

// ── imbl::HashMap ────────────────────────────────────────────────

impl<K, V> BenchMap<K, V> for ImHashMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    type Iter<'a>
        = imbl::hashmap::Iter<'a, K, V, imbl::shared_ptr::DefaultSharedPtr>
    where
        K: 'a,
        V: 'a;

    fn new() -> Self {
        ImHashMap::new()
    }
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
    fn insert_clone(&self, k: K, v: V) -> Self {
        self.update(k, v)
    }
    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(k)
    }
    fn remove_clone(&self, k: &K) -> Self {
        self.without(k)
    }
    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

// ── std::collections::HashMap ────────────────────────────────────

impl<K, V> BenchMap<K, V> for StdHashMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    type Iter<'a>
        = std::collections::hash_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn new() -> Self {
        StdHashMap::new()
    }
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
    fn insert_clone(&self, k: K, v: V) -> Self {
        let mut m = self.clone();
        m.insert(k, v);
        m
    }
    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(k)
    }
    fn remove_clone(&self, k: &K) -> Self {
        let mut m = self.clone();
        m.remove(k);
        m
    }
    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn has_expensive_clone() -> bool {
        true
    }
}

impl<K, V> BenchMap<K, V> for CrateIndexMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    type Iter<'a>
        = indexmap::map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn new() -> Self {
        CrateIndexMap::new()
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }

    fn insert_clone(&self, k: K, v: V) -> Self {
        let mut m = self.clone();
        m.insert(k, v);
        m
    }

    fn remove(&mut self, k: &K) -> Option<V> {
        self.shift_remove(k)
    }

    fn remove_clone(&self, k: &K) -> Self {
        let mut m = self.clone();
        m.shift_remove(k);
        m
    }

    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn has_expensive_clone() -> bool {
        true
    }

    fn has_expensive_remove() -> bool {
        true
    }
}

// ── imbl::OrdMap ─────────────────────────────────────────────────

impl<K, V> BenchMap<K, V> for ImOrdMap<K, V>
where
    K: Clone + Ord + Hash + Eq,
    V: Clone,
{
    type Iter<'a>
        = imbl::ordmap::Iter<'a, K, V, imbl::shared_ptr::DefaultSharedPtr>
    where
        K: 'a,
        V: 'a;

    fn new() -> Self {
        ImOrdMap::new()
    }
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
    fn insert_clone(&self, k: K, v: V) -> Self {
        self.update(k, v)
    }
    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(k)
    }
    fn remove_clone(&self, k: &K) -> Self {
        self.without(k)
    }
    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

// ── imbl_index::IndexMap ─────────────────────────────────────────

impl<K, V> BenchMap<K, V>
    for GenericIndexMap<
        K,
        V,
        std::collections::hash_map::RandomState,
        imbl::shared_ptr::DefaultSharedPtr,
    >
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    type Iter<'a>
        = imbl_index::Iter<'a, K, V, imbl::shared_ptr::DefaultSharedPtr>
    where
        K: 'a,
        V: 'a;

    fn new() -> Self {
        IndexMap::new()
    }
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }
    fn insert_clone(&self, k: K, v: V) -> Self {
        self.update(k, v)
    }
    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(k)
    }
    fn remove_clone(&self, k: &K) -> Self {
        self.without(k)
    }
    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

// ── Generic benchmark functions ──────────────────────────────────

fn prepare_map<M, K, V, F>(size: usize, rng_seed: u64, filter: F) -> M
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
    F: FnMut(&(usize, (K, V))) -> bool,
{
    let mut keys = K::generate(size);
    let mut values = V::generate(size);

    // Hopefully reordering same-sized arrays with the same rng_seed produces the same results!
    // A valid question would be whether we even want to. We might even want them to be separate
    reorder_in_place(&mut keys, rng_seed);
    reorder_in_place(&mut values, rng_seed);

    keys.into_iter()
        .zip(values)
        .enumerate()
        .filter(filter)
        .map(|(_, kv)| kv)
        .collect()
}

fn partially_shuffle_map<M, K, V>(map: &mut M, shuffle_count: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    use rand::prelude::*;

    let allkeys: Vec<_> = map.keys_cloned().collect();

    let mut rng = SmallRng::seed_from_u64(1);
    let sampled_keys = allkeys.choose_multiple(&mut rng, shuffle_count).cloned();

    for key in sampled_keys {
        let v = map.remove(&key).unwrap();
        map.insert(key, v);
    }
}

fn bench_lookup<M, K, V>(b: &mut Bencher, size: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    let keys = K::generate(size);
    let values = V::generate(size);
    let order = reorder(&keys);
    let m: M = keys.into_iter().zip(values).collect();
    b.iter(|| {
        for k in &order {
            black_box(m.get(k));
        }
    })
}

fn bench_insert<M, K, V>(b: &mut Bencher, size: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    let keys = K::generate(size);
    let values = V::generate(size);
    b.iter(|| {
        let mut m = M::new();
        for (k, v) in keys.clone().into_iter().zip(values.clone()) {
            m = m.insert_clone(k, v);
        }
        m
    })
}

fn bench_insert_mut<M, K, V>(b: &mut Bencher, size: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    let keys = K::generate(size);
    let values = V::generate(size);
    b.iter(|| {
        let mut m = M::new();
        for (k, v) in keys.clone().into_iter().zip(values.clone()) {
            m.insert(k, v);
        }
        m
    })
}

fn bench_remove<M, K, V>(b: &mut Bencher, size: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    let keys = K::generate(size);
    let values = V::generate(size);
    let order = reorder(&keys);
    let map: M = keys.into_iter().zip(values).collect();
    b.iter(|| {
        let mut m = map.clone();
        for k in &order {
            m = m.remove_clone(k);
        }
        m
    })
}

fn bench_remove_mut<M, K, V>(b: &mut Bencher, size: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    let keys = K::generate(size);
    let values = V::generate(size);
    let order = reorder(&keys);
    let map: M = keys.into_iter().zip(values).collect();
    b.iter(|| {
        let mut m = map.clone();
        for k in &order {
            m.remove(k);
        }
        m
    })
}

fn bench_iter<M, K, V>(b: &mut Bencher, size: usize)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    let keys = K::generate(size);
    let values = V::generate(size);
    let m: M = keys.into_iter().zip(values).collect();
    b.iter(|| {
        for p in m.iter() {
            black_box(p);
        }
    })
}

// ── Benchmark groups ─────────────────────────────────────────────

fn bench_group<M, K, V>(c: &mut Criterion, group_name: &str)
where
    M: BenchMap<K, V>,
    K: TestData,
    V: TestData,
{
    use rand::prelude::*;

    let mut group = c.benchmark_group(group_name);

    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));
    group.warm_up_time(std::time::Duration::from_secs(2));

    // Size for tests that repeatedly clone when constructing the map
    let clone_sizes = if M::has_expensive_clone() {
        &[100, 1000][..]
    } else {
        &[100, 1000, 5000, 10000][..]
    };

    // Size for tests that build the map once per iter
    let mut_sizes = &[100, 1000, 5000, 10000][..];

    // Size for tests that build the map once, but shift O(n) on removes
    let mut_remove_sizes = if M::has_expensive_remove() {
        &[100, 1000][..]
    } else {
        &[100, 1000, 5000, 10000][..]
    };

    // Size for tests that only build the map once and do one constant-time operation on it
    // NOTE: CrateIndexMap probably needs a has_expensive_remove check for the reinsert test
    let one_sizes = &[100, 1000, 5000, 10000, 50000, 100000][..];

    for size in mut_sizes {
        group.bench_function(format!("lookup_{}", size), |b| {
            bench_lookup::<M, K, V>(b, *size);
        });
    }

    for size in one_sizes {
        // Lazy init the big maps
        let mut m: Option<M> = None;
        let mut k: Option<Vec<_>> = None;

        // Reuse rng between iters to lookup different keys
        let mut rng = SmallRng::seed_from_u64(2);

        group.bench_function(format!("lookup_one_{}", size), |b| {
            let map = m.get_or_insert_with(|| prepare_map(*size, 1, |_| true));
            let keys = k.get_or_insert_with(|| map.keys_cloned().collect());

            b.iter(|| {
                let k = keys.choose(&mut rng).unwrap();

                black_box(map.get(k));
            });
        });
    }

    for size in clone_sizes {
        group.bench_function(format!("insert_{}", size), |b| {
            bench_insert::<M, K, V>(b, *size);
        });
    }

    for size in clone_sizes {
        let mut m: Option<M> = None;

        let mut rng = SmallRng::seed_from_u64(2);

        group.bench_function(format!("insert_one_{}", size), |b| {
            let map = m.get_or_insert_with(|| {
                prepare_map(size * 2, 1, |(index, _)| index.is_multiple_of(2))
            });

            b.iter(|| {
                let index = rng.random_range(0..*size) * 2;

                let k = K::generate_one(index);
                let v = V::generate_one(index);

                black_box(map.insert_clone(k, v));
            });
        });
    }

    for size in mut_sizes {
        group.bench_function(format!("insert_mut_{}", size), |b| {
            bench_insert_mut::<M, K, V>(b, *size);
        });
    }

    for size in one_sizes {
        let mut m: Option<M> = None;
        let mut k: Option<Vec<_>> = None;

        let mut rng = SmallRng::seed_from_u64(2);

        group.bench_function(format!("reinsert_mut_one_{}", size), |b| {
            let map = m.get_or_insert_with(|| prepare_map(*size, 1, |_| true));
            let keys = k.get_or_insert_with(|| map.keys_cloned().collect());

            partially_shuffle_map(map, 100);

            b.iter(|| {
                let k = keys.choose(&mut rng).unwrap().clone();
                let v = map.remove(&k).unwrap();

                black_box(map.insert(k, v));
            });
        });
    }

    for size in clone_sizes {
        group.bench_function(format!("remove_{}", size), |b| {
            bench_remove::<M, K, V>(b, *size);
        });
    }

    for size in mut_remove_sizes {
        group.bench_function(format!("remove_mut_{}", size), |b| {
            bench_remove_mut::<M, K, V>(b, *size);
        });
    }

    for size in mut_sizes {
        group.bench_function(format!("iter_{}", size), |b| {
            bench_iter::<M, K, V>(b, *size);
        });
    }

    group.finish();
}

// ── IndexMap-specific benchmarks ─────────────────────────────────

fn bench_indexmap_specific(c: &mut Criterion) {
    use rand::prelude::*;

    let mut group = c.benchmark_group("indexmap_im_specific_i64");

    let mut rng = SmallRng::seed_from_u64(2);

    for size in &[100, 1000, 5000, 10000, 50000, 100000, 500000, 1000000] {
        let keys = i64::generate(*size);
        let values = i64::generate(*size);

        group.bench_function(format!("get_index_{}", size), |b| {
            let m: IndexMap<i64, i64> = keys.clone().into_iter().zip(values.clone()).collect();
            b.iter(|| {
                let lookup_index = rng.random_range(0..*size);

                black_box(m.get_index(lookup_index));
            });
        });

        group.bench_function(format!("first_{}", size), |b| {
            let m: IndexMap<i64, i64> = keys.clone().into_iter().zip(values.clone()).collect();
            b.iter(|| {
                black_box(m.first());
            });
        });

        group.bench_function(format!("last_{}", size), |b| {
            let m: IndexMap<i64, i64> = keys.clone().into_iter().zip(values.clone()).collect();
            b.iter(|| {
                black_box(m.last());
            });
        });
    }

    group.finish();

    let mut group = c.benchmark_group("indexmap_crate_specific_i64");

    for size in &[100, 1000, 5000, 10000, 50000, 100000, 500000, 1000000] {
        let keys = i64::generate(*size);
        let values = i64::generate(*size);

        group.bench_function(format!("get_index_{}", size), |b| {
            let m: CrateIndexMap<i64, i64> = keys.clone().into_iter().zip(values.clone()).collect();
            b.iter(|| {
                let lookup_index = rng.random_range(0..*size);

                black_box(m.get_index(lookup_index));
            });
        });

        group.bench_function(format!("first_{}", size), |b| {
            let m: CrateIndexMap<i64, i64> = keys.clone().into_iter().zip(values.clone()).collect();
            b.iter(|| {
                black_box(m.first());
            });
        });

        group.bench_function(format!("last_{}", size), |b| {
            let m: CrateIndexMap<i64, i64> = keys.clone().into_iter().zip(values.clone()).collect();
            b.iter(|| {
                black_box(m.last());
            });
        });
    }

    group.finish();
}

// ── i64 benchmarks ───────────────────────────────────────────────

fn bench_indexmap_im_i64(c: &mut Criterion) {
    bench_group::<IndexMap<i64, i64>, i64, i64>(c, "indexmap_im_i64");
}

fn bench_indexmap_i64(c: &mut Criterion) {
    bench_group::<CrateIndexMap<i64, i64>, i64, i64>(c, "indexmap_crate_i64");
}

fn bench_hashmap_std_i64(c: &mut Criterion) {
    bench_group::<StdHashMap<i64, i64>, i64, i64>(c, "hashmap_std_i64");
}

fn bench_hashmap_im_i64(c: &mut Criterion) {
    bench_group::<ImHashMap<i64, i64>, i64, i64>(c, "hashmap_im_i64");
}

fn bench_ordmap_i64(c: &mut Criterion) {
    bench_group::<ImOrdMap<i64, i64>, i64, i64>(c, "ordmap_i64");
}

// ── String benchmarks ────────────────────────────────────────────

fn bench_indexmap_im_str(c: &mut Criterion) {
    bench_group::<IndexMap<String, String>, String, String>(c, "indexmap_im_str");
}

fn bench_indexmap_str(c: &mut Criterion) {
    bench_group::<CrateIndexMap<String, String>, String, String>(c, "indexmap_crate_str");
}

fn bench_hashmap_std_str(c: &mut Criterion) {
    bench_group::<StdHashMap<String, String>, String, String>(c, "hashmap_std_str");
}

fn bench_hashmap_im_str(c: &mut Criterion) {
    bench_group::<ImHashMap<String, String>, String, String>(c, "hashmap_im_str");
}

fn bench_ordmap_str(c: &mut Criterion) {
    bench_group::<ImOrdMap<String, String>, String, String>(c, "ordmap_str");
}

// ── BigType benchmarks ───────────────────────────────────────────

fn bench_indexmap_im_big(c: &mut Criterion) {
    bench_group::<IndexMap<BigType, BigType>, BigType, BigType>(c, "indexmap_im_big");
}

fn bench_indexmap_big(c: &mut Criterion) {
    bench_group::<CrateIndexMap<BigType, BigType>, BigType, BigType>(c, "indexmap_crate_big");
}

fn bench_hashmap_std_big(c: &mut Criterion) {
    bench_group::<StdHashMap<BigType, BigType>, BigType, BigType>(c, "hashmap_std_big");
}

fn bench_hashmap_im_big(c: &mut Criterion) {
    bench_group::<ImHashMap<BigType, BigType>, BigType, BigType>(c, "hashmap_im_big");
}

fn bench_ordmap_big(c: &mut Criterion) {
    bench_group::<ImOrdMap<BigType, BigType>, BigType, BigType>(c, "ordmap_big");
}

// ── Entry point ──────────────────────────────────────────────────

fn indexmap_benches(c: &mut Criterion) {
    bench_indexmap_im_i64(c);
    bench_indexmap_i64(c);

    if std::env::var("BENCH_STD").is_ok() {
        bench_hashmap_std_i64(c);
    }

    bench_hashmap_im_i64(c);
    bench_ordmap_i64(c);
    bench_indexmap_im_str(c);
    bench_indexmap_str(c);

    if std::env::var("BENCH_STD").is_ok() {
        bench_hashmap_std_str(c);
    }

    bench_hashmap_im_str(c);
    bench_ordmap_str(c);
    bench_indexmap_im_big(c);
    bench_indexmap_big(c);

    if std::env::var("BENCH_STD").is_ok() {
        bench_hashmap_std_big(c);
    }

    bench_hashmap_im_big(c);
    bench_ordmap_big(c);
    bench_indexmap_specific(c);
}

criterion_group!(benches, indexmap_benches);
criterion_main!(benches);
