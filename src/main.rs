mod error;

use crdts::{CmRDT, CvRDT, List, Map, Orswot, VClock};
use error::CrdtError;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Controller {
    clock: VClock<u64>,
    txns: Orswot<&'static str, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            txns: Orswot::new(),
            history_hashes: Map::new(),
            candidates: Orswot::new(),
            clock: Default::default(),
        }
    }
}

impl CmRDT for Controller {
    type Op = String;
    type Validation = CrdtError;

    fn apply(&mut self, op: Self::Op) {
        todo!()
    }

    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation> {
        todo!()
    }
}

impl CvRDT for Controller {
    type Validation = CrdtError;

    fn validate_merge(&self, other: &Self) -> Result<(), Self::Validation> {
        todo!()
    }

    fn merge(&mut self, other: Self) {
        todo!()
    }
}

fn main() {}

#[test]
fn test() {
    use crdts::{CmRDT, Orswot};
    let mut set: Orswot<u8, &'static str> = Default::default();
    let add_ctx = set.read_ctx().derive_add_ctx("actor");
    set.apply(set.add(50, add_ctx));
    let add_ctx = set.read_ctx().derive_add_ctx("actor1");
    set.apply(set.add(50, add_ctx));
    let mut items: Vec<_> = set.iter().map(|item_ctx| *item_ctx.val).collect();
    items.sort();
    assert_eq!(items, &[50]);
}
