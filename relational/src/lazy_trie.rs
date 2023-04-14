use std::collections::{BTreeMap, HashMap};
use smallvec::SmallVec;
use arc_symbol::Symbol;

enum TrieVariant<S, T> {
    Lazy(SmallVec<[usize; 4]>),
    Sparse(HashMap<T, Self>),
    Flat(Vec<T>),
    Borrowed {
        column: Symbol<S>,
        trie: HashMap<T, Self>
    },
}