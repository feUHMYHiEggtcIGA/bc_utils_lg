#![allow(non_camel_case_types)]

use crate::enums::indicators::*;
use crate::types::maps::MAP;


pub type SRC_EL<T> = MAP<String, T>;
pub type SRC<T> = Vec<SRC_EL<T>>;
pub type SRC_TRANSPOSE<T> = MAP<String, Vec<T>>;
pub type SLICE_ARG<T> = [T];
pub type SLICE1_ARG<'a, T> = [&'a SLICE_ARG<T>];
pub type SLICE2_ARG<'a, T> = [&'a SLICE1_ARG<'a, T>];
pub type BF_VEC<T> = Vec<T_HASHMAP<T>>;
pub type ARGS<'a, F> = Vec<T_ARGS<'a, F>>;
