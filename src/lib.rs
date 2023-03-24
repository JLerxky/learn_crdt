pub mod error;
pub mod one;
pub mod one_by_one;

use crdts::Orswot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpOneByOne {
    T(crdts::orswot::Op<String, u64>),
    H(crdts::map::Op<u64, Orswot<Vec<u8>, u64>, u64>),
    C(crdts::orswot::Op<Vec<u8>, u64>),
}

pub type OpAllInOne = (
    Option<crdts::orswot::Op<String, u64>>,
    Option<crdts::map::Op<u64, Orswot<Vec<u8>, u64>, u64>>,
    Option<crdts::orswot::Op<Vec<u8>, u64>>,
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpMix {
    TaHuCa,
    TrHuCa,
    TaHrCa,
    TaHuCr,
    // ...
}
