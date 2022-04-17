pub mod utils {
    use std::{collections::HashMap, hash::Hash, marker::PhantomData, ops::Sub};

    use num::{one, zero, Unsigned};

    pub trait FactorialsCache<I> {
        fn new() -> Self;
        fn get(&self, n: I) -> Option<I>;
        fn set(&mut self, n: I, v: I);
        fn last_set(&self) -> I;
    }

    pub struct FactorialsHashMap<I> {
        inner: HashMap<I, I>,
    }

    #[derive(Default)]
    pub struct CachedFactorials<I: Hash + Eq + Clone, C: FactorialsCache<I>> {
        pub phantom: PhantomData<I>,
        pub cache: C,
    }

    impl<I: Unsigned + Hash + Eq + Clone + From<usize>> FactorialsCache<I> for FactorialsHashMap<I> {
        fn new() -> Self {
            let mut inner = HashMap::new();
            inner.insert(zero(), one());
            inner.insert(one(), one());
            FactorialsHashMap { inner }
        }

        fn get(&self, n: I) -> Option<I> {
            self.inner.get(&n).map(|x| x.clone())
        }

        fn set(&mut self, n: I, v: I) {
            self.inner.insert(n, v);
        }

        fn last_set(&self) -> I {
            self.inner.len().to_owned().sub(1).into()
        }
    }

    pub struct NoCacheCache<I> {
        phantom: PhantomData<I>,
    }

    impl<I: Unsigned + Hash + Eq + Clone + From<usize>> FactorialsCache<I> for NoCacheCache<I> {
        fn new() -> Self {
            NoCacheCache {
                phantom: PhantomData,
            }
        }

        fn get(&self, _n: I) -> Option<I> {
            None
        }

        fn set(&mut self, _n: I, _v: I) {}

        fn last_set(&self) -> I {
            one()
        }
    }
}
