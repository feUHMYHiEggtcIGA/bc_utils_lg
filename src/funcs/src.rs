use std::error::Error;

use num_traits::Float;

use crate::types::structures::{
    SRC,
    SRC_TRANSPOSE
};
use crate::types::maps::MAP;


pub fn transpose<T>(src: SRC<T>) -> Result<SRC_TRANSPOSE<T>, Box<dyn Error>>
where
    T: Float
    {
        Ok(MAP::from_iter(
            src
                .first()
                .ok_or(Box::<dyn Error>::from("first get err"))
                ?
                .iter()
                .map(|v| {
                    let key = v.0;
                    (key.clone(), src.iter().map(|v1| v1[key]).collect::<Vec<T>>())
                })
                .collect::<SRC_TRANSPOSE<T>>()
        ))
    }
