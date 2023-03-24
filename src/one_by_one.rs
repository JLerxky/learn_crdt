use core::fmt::Debug;

use crate::{error::CrdtError, OpOneByOne};
use crdts::{CmRDT, CvRDT, Dot, Map, Orswot, VClock};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControllerOneByOne {
    clock: VClock<u64>,
    txns: Orswot<String, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
}

impl ControllerOneByOne {
    pub fn new() -> Self {
        Self {
            clock: Default::default(),
            txns: Orswot::new(),
            history_hashes: Map::new(),
            candidates: Orswot::new(),
        }
    }
}

pub struct Op {
    dot: Dot<u64>,
    op: OpOneByOne,
}

impl CmRDT for ControllerOneByOne {
    type Op = Op;
    type Validation = CrdtError;

    fn apply(&mut self, op: Self::Op) {
        if self.clock.get(&op.dot.actor) >= op.dot.counter {
            return;
        }
        match op.op {
            OpOneByOne::T(o) => self.txns.apply(o),
            OpOneByOne::H(o) => self.history_hashes.apply(o),
            OpOneByOne::C(o) => self.candidates.apply(o),
        }
        self.clock.apply(op.dot);
    }

    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation> {
        self.clock.validate_op(&op.dot).map_err(CrdtError::VClock)?;
        match &op.op {
            OpOneByOne::T(o) => self.txns.validate_op(o).map_err(CrdtError::VClock),
            OpOneByOne::H(op) => self.history_hashes.validate_op(op).map_err(CrdtError::Map),
            OpOneByOne::C(op) => self.candidates.validate_op(op).map_err(CrdtError::VClock),
        }
    }
}

impl CvRDT for ControllerOneByOne {
    type Validation = CrdtError;

    fn validate_merge(&self, _other: &Self) -> Result<(), Self::Validation> {
        todo!()
    }

    fn merge(&mut self, _other: Self) {
        todo!()
    }
}

#[test]
fn test() {
    let mut controller = ControllerOneByOne::new();
    let op1 = controller.txns.add(
        "member".to_owned(),
        controller.txns.read().derive_add_ctx(9_742_820),
    );
    controller.apply(Op {
        dot: Dot::new(9_742_820, 2),
        op: OpOneByOne::T(op1),
    });
    let op2 = controller.candidates.add(
        vec![8; 20],
        controller.txns.read().derive_add_ctx(9_742_820),
    );
    controller.apply(Op {
        dot: Dot::new(9_742_820, 3),
        op: OpOneByOne::C(op2),
    });
    let add_ctx = controller
        .history_hashes
        .read_ctx()
        .derive_add_ctx(9_742_820);
    let op3 = controller
        .history_hashes
        .update(10u64, add_ctx, |v, a| v.add(vec![8; 20], a));
    controller.apply(Op {
        dot: Dot::new(9_742_820, 4),
        op: OpOneByOne::H(op3),
    });
    println!("{:#?}", controller);
}
