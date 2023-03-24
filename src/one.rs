use core::fmt::Debug;

use crate::{error::CrdtError, OpAllInOne};
use crdts::{CmRDT, CvRDT, Dot, Map, Orswot, VClock};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControllerAllInOne {
    clock: VClock<u64>,
    txns: Orswot<String, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
}

impl ControllerAllInOne {
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
    op: OpAllInOne,
}

impl CmRDT for ControllerAllInOne {
    type Op = Op;
    type Validation = CrdtError;

    fn apply(&mut self, op: Self::Op) {
        if self.clock.get(&op.dot.actor) >= op.dot.counter {
            return;
        }
        match op.op {
            (None, None, None) => return,
            (t_op, h_op, c_op) => {
                if let Some(t_op) = t_op {
                    self.txns.apply(t_op);
                }
                if let Some(h_op) = h_op {
                    self.history_hashes.apply(h_op);
                }
                if let Some(c_op) = c_op {
                    self.candidates.apply(c_op);
                }
            }
        }
        self.clock.apply(op.dot);
    }

    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation> {
        self.clock.validate_op(&op.dot).map_err(CrdtError::VClock)?;
        match &op.op {
            (None, None, None) => return Err(CrdtError::NoneOp),
            (t_op, h_op, c_op) => {
                if let Some(t_op) = t_op {
                    return self.txns.validate_op(t_op).map_err(CrdtError::VClock);
                }
                if let Some(h_op) = h_op {
                    return self
                        .history_hashes
                        .validate_op(h_op)
                        .map_err(CrdtError::Map);
                }
                if let Some(c_op) = c_op {
                    return self.candidates.validate_op(c_op).map_err(CrdtError::VClock);
                }
            }
        }
        Err(CrdtError::NoneOp)
    }
}

impl CvRDT for ControllerAllInOne {
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
    let mut controller = ControllerAllInOne::new();

    let op1 = controller.txns.add(
        "member".to_owned(),
        controller.txns.read().derive_add_ctx(9_742_820),
    );

    let add_ctx = controller
        .history_hashes
        .read_ctx()
        .derive_add_ctx(9_742_820);
    let op2 = controller
        .history_hashes
        .update(10u64, add_ctx, |v, a| v.add(vec![8; 20], a));

    let op3 = controller.candidates.add(
        vec![8; 20],
        controller.txns.read().derive_add_ctx(9_742_820),
    );
    controller.apply(Op {
        dot: Dot::new(9_742_820, 4),
        op: (Some(op1), Some(op2), Some(op3)),
    });
    println!("{:#?}", controller);
}
