use core::fmt::Debug;

use crdts::{CmRDT, CvRDT, Dot, Map, Orswot, VClock};
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Controller {
    v_clock: VClock<u64>,
    txns: Orswot<String, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
}

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("none op")]
    NoneOp,
    #[error("VClock error")]
    VClock(<VClock<u64> as CmRDT>::Validation),
    #[error("Txns error")]
    Txns(<Orswot<String, u64> as CmRDT>::Validation),
    #[error("HistoryHashes error")]
    HistoryHashes(<Map<u64, Orswot<Vec<u8>, u64>, u64> as CmRDT>::Validation),
    #[error("Candidates error")]
    Candidates(<Orswot<Vec<u8>, u64> as CmRDT>::Validation),
}

#[allow(clippy::type_complexity)]
pub struct Op {
    dot: Dot<u64>,
    txns_op: Option<<Orswot<String, u64> as CmRDT>::Op>,
    history_hashes_op: Option<<Map<u64, Orswot<Vec<u8>, u64>, u64> as CmRDT>::Op>,
    candidates_op: Option<<Orswot<Vec<u8>, u64> as CmRDT>::Op>,
}

impl CmRDT for Controller {
    type Op = Op;
    type Validation = ControllerError;

    fn apply(&mut self, op: Self::Op) {
        let Op {
            dot,
            txns_op,
            history_hashes_op,
            candidates_op,
        } = op;
        if self.v_clock.get(&dot.actor) >= dot.counter {
            return;
        }
        match (txns_op, history_hashes_op, candidates_op) {
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
        self.v_clock.apply(dot);
    }

    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation> {
        let Op {
            dot,
            txns_op,
            history_hashes_op,
            candidates_op,
        } = op;
        self.v_clock
            .validate_op(dot)
            .map_err(ControllerError::VClock)?;
        match &(txns_op, history_hashes_op, candidates_op) {
            (None, None, None) => Err(ControllerError::NoneOp),
            (t_op, h_op, c_op) => {
                if let Some(t_op) = t_op {
                    self.txns.validate_op(t_op).map_err(ControllerError::Txns)?;
                }
                if let Some(h_op) = h_op {
                    self.history_hashes
                        .validate_op(h_op)
                        .map_err(ControllerError::HistoryHashes)?;
                }
                if let Some(c_op) = c_op {
                    self.candidates
                        .validate_op(c_op)
                        .map_err(ControllerError::Candidates)?;
                }
                Ok(())
            }
        }
    }
}

impl CvRDT for Controller {
    type Validation = ControllerError;

    fn validate_merge(&self, _other: &Self) -> Result<(), Self::Validation> {
        todo!()
    }

    fn merge(&mut self, _other: Self) {
        todo!()
    }
}

#[test]
fn test() {
    let mut controller = Controller::default();
    let actor = 9_742_820;
    let counter = 1;
    let dot = Dot::new(actor, counter);
    let op1 = controller.txns.add(
        "member".to_owned(),
        controller.txns.read().derive_add_ctx(actor),
    );

    let add_ctx = controller.history_hashes.read_ctx().derive_add_ctx(actor);
    let op2 = controller
        .history_hashes
        .update(10u64, add_ctx, |v, a| v.add(vec![8; 20], a));

    let op3 = controller
        .candidates
        .add(vec![8; 20], controller.txns.read().derive_add_ctx(actor));
    controller.apply(Op {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
    });
    println!("{:#?}", controller);

    let actor = 9_742_820;
    let counter = 2;
    let dot = Dot::new(actor, counter);
    let op1 = controller.txns.add(
        "member".to_owned(),
        controller.txns.read().derive_add_ctx(actor),
    );

    let add_ctx = controller.history_hashes.read_ctx().derive_add_ctx(actor);
    let op2 = controller
        .history_hashes
        .update(10u64, add_ctx, |v, a| v.add(vec![8; 20], a));

    let op3 = controller
        .candidates
        .add(vec![8; 20], controller.txns.read().derive_add_ctx(actor));
    controller.apply(Op {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
    });
    println!("{:#?}", controller);
}
