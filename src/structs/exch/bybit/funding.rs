#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt::Debug as Debugg;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debugg)]
pub struct RESULT_SYMBOLS1
{
    pub symbol: String,
    pub fundingRate: String,
    pub fundingRateTimestamp: String,
}

#[derive(Serialize, Deserialize, Debugg)]
pub struct RESULT_SYMBOLS
{
    pub category: String,
    pub list: Vec<RESULT_SYMBOLS1>
}
