mod error;

use core::fmt::Debug;

use crdts::{CmRDT, CvRDT, Dot, Map, Orswot, VClock};
use error::CrdtError;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Controller {
    clock: VClock<u64>,
    txns: Orswot<String, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
}

impl Controller {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpOneByOne {
    T(crdts::orswot::Op<String, u64>),
    H(crdts::map::Op<u64, Orswot<Vec<u8>, u64>, u64>),
    C(crdts::orswot::Op<Vec<u8>, u64>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpAllInOne {
    All(
        Option<crdts::orswot::Op<String, u64>>,
        Option<crdts::map::Op<u64, Orswot<Vec<u8>, u64>, u64>>,
        Option<crdts::orswot::Op<Vec<u8>, u64>>,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpMix {
    TaHuCa,
    TrHuCa,
    TaHrCa,
    TaHuCr,
    // ...
}

impl CmRDT for Controller {
    type Op = Op;
    type Validation = CrdtError;

    fn apply(&mut self, op: Self::Op) {
        match op.op {
            OpOneByOne::T(o) => {
                if self.clock.get(&op.dot.actor) >= op.dot.counter {
                    return;
                }
                self.txns.apply(o);

                self.clock.apply(op.dot);
            }
            OpOneByOne::H(o) => {
                if self.clock.get(&op.dot.actor) >= op.dot.counter {
                    return;
                }
                self.history_hashes.apply(o);

                self.clock.apply(op.dot);
            }
            OpOneByOne::C(o) => {
                if self.clock.get(&op.dot.actor) >= op.dot.counter {
                    return;
                }
                self.candidates.apply(o);

                self.clock.apply(op.dot);
            }
        }
    }

    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation> {
        match &op.op {
            OpOneByOne::T(op) => match &op {
                crdts::orswot::Op::Add { dot, .. } => {
                    self.clock.validate_op(dot).map_err(CrdtError::VClock)?;
                    self.txns.validate_op(op).map_err(CrdtError::VClock)
                }
                crdts::orswot::Op::Rm { .. } => Ok(()),
            },
            OpOneByOne::H(op) => {
                match &op {
                    crdts::map::Op::Rm { .. } => {}
                    crdts::map::Op::Up { dot, .. } => {
                        self.clock.validate_op(dot).map_err(CrdtError::VClock)?;
                    }
                }
                self.history_hashes.validate_op(op).map_err(CrdtError::Map)
            }
            OpOneByOne::C(op) => match &op {
                crdts::orswot::Op::Add { dot, .. } => {
                    self.clock.validate_op(dot).map_err(CrdtError::VClock)?;
                    self.candidates.validate_op(op).map_err(CrdtError::VClock)
                }
                crdts::orswot::Op::Rm { .. } => Ok(()),
            },
        }
    }
}

impl CvRDT for Controller {
    type Validation = CrdtError;

    fn validate_merge(&self, _other: &Self) -> Result<(), Self::Validation> {
        todo!()
    }

    fn merge(&mut self, _other: Self) {
        todo!()
    }
}

fn main() {}

#[test]
fn test() {
    let mut controller = Controller::new();
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
