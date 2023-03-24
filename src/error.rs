use crdts::{map::CmRDTValidation, CmRDT, Orswot, VClock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrdtError {
    #[error("unknown error")]
    Unknown,
    #[error("VClock error")]
    VClock(<VClock<u64> as CmRDT>::Validation),
    #[error("Map error")]
    Map(CmRDTValidation<Orswot<Vec<u8>, u64>, u64>),
}
