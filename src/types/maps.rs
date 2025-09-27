#![allow(non_camel_case_types)]

use rustc_hash::{
    FxHasher,
    FxHashMap,
};
use core::hash::BuildHasherDefault;
use indexmap::IndexMap;

use crate::types::funcs::*;
use crate::types::structures::*;


pub type MAP<K, V> = FxHashMap<K, V>;
pub type MAP_LINK<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;


pub type MAP_BF_VEC<'a, T> = FxHashMap<&'a str, BF_VEC<T>>;
pub type MAP1_BF_VEC<'a, T> = FxHashMap<&'a str, MAP_BF_VEC<'a, T>>;
pub type MAP_IND_T<'a, F> = FxHashMap<&'a str, IND_T<F>>;
pub type MAP_FUNC_T<'a, F> = FxHashMap<&'a str, FUNC_T<F>>;
pub type MAP_IND_COLL<'a, C, F> = FxHashMap<&'a str, IND_COLL<C, F>>;
pub type MAP_IND_T_BF<'a, F> = FxHashMap<&'a str, IND_T_BF<F>>;
pub type MAP_FUNC_BF_IND<'a, F> = FxHashMap<&'a str, FUNC_BF_IND<'a, F>>;
pub type MAP_ARGS<'a, F> = FxHashMap<&'a str, ARGS<'a, F>>;
pub type MAP1_ARGS<'a, F> = FxHashMap<&'a str, MAP_ARGS<'a, F>>;
pub type MAP2_ARGS<'a, F> = FxHashMap<&'a str, MAP1_ARGS<'a, F>>;
pub type MAP_FUNC_USIZE<'a, F> = FxHashMap<&'a str, FUNC_USIZE<F>>;
pub type MAP_USIZE = FxHashMap<&'static str, usize>;
pub type MAP_COLL<'a, C> = FxHashMap<&'a str, C>;
