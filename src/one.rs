use core::fmt::Debug;

use crdts::{CmRDT, CvRDT, Dot, Map, Orswot, VClock};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Controller {
    v_clock: VClock<u64>,
    txns: Orswot<String, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ControllerCmRDTError {
    NoneOp,
    VClock(<VClock<u64> as CmRDT>::Validation),
    Txns(<Orswot<String, u64> as CmRDT>::Validation),
    HistoryHashes(<Map<u64, Orswot<Vec<u8>, u64>, u64> as CmRDT>::Validation),
    Candidates(<Orswot<Vec<u8>, u64> as CmRDT>::Validation),
}

impl std::fmt::Display for ControllerCmRDTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for ControllerCmRDTError {}

#[derive(Debug, PartialEq, Eq)]
pub enum ControllerCvRDTError {
    VClock(<VClock<u64> as CvRDT>::Validation),
    Txns(<Orswot<String, u64> as CvRDT>::Validation),
    HistoryHashes(<Map<u64, Orswot<Vec<u8>, u64>, u64> as CvRDT>::Validation),
    Candidates(<Orswot<Vec<u8>, u64> as CvRDT>::Validation),
}

impl std::fmt::Display for ControllerCvRDTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl std::error::Error for ControllerCvRDTError {}

#[allow(clippy::type_complexity)]
pub struct Op {
    dot: Dot<u64>,
    txns_op: Option<<Orswot<String, u64> as CmRDT>::Op>,
    history_hashes_op: Option<<Map<u64, Orswot<Vec<u8>, u64>, u64> as CmRDT>::Op>,
    candidates_op: Option<<Orswot<Vec<u8>, u64> as CmRDT>::Op>,
}

impl CmRDT for Controller {
    type Op = Op;
    type Validation = ControllerCmRDTError;

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
            .map_err(ControllerCmRDTError::VClock)?;
        match &(txns_op, history_hashes_op, candidates_op) {
            (None, None, None) => Err(ControllerCmRDTError::NoneOp),
            (t_op, h_op, c_op) => {
                if let Some(t_op) = t_op {
                    self.txns
                        .validate_op(t_op)
                        .map_err(ControllerCmRDTError::Txns)?;
                }
                if let Some(h_op) = h_op {
                    self.history_hashes
                        .validate_op(h_op)
                        .map_err(ControllerCmRDTError::HistoryHashes)?;
                }
                if let Some(c_op) = c_op {
                    self.candidates
                        .validate_op(c_op)
                        .map_err(ControllerCmRDTError::Candidates)?;
                }
                Ok(())
            }
        }
    }
}

impl CvRDT for Controller {
    type Validation = ControllerCvRDTError;

    fn validate_merge(&self, other: &Self) -> Result<(), Self::Validation> {
        self.v_clock
            .validate_merge(&other.v_clock)
            .map_err(Self::Validation::VClock)?;
        self.txns
            .validate_merge(&other.txns)
            .map_err(Self::Validation::Txns)?;
        self.history_hashes
            .validate_merge(&other.history_hashes)
            .map_err(Self::Validation::HistoryHashes)?;
        self.candidates
            .validate_merge(&other.candidates)
            .map_err(Self::Validation::Candidates)?;
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        self.v_clock.merge(other.v_clock);
        self.txns.merge(other.txns);
        self.history_hashes.merge(other.history_hashes);
        self.candidates.merge(other.candidates);
    }
}

#[test]
fn test_cmrdt() {
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

#[test]
fn test_cvrdt() {
    let mut controller1 = Controller::default();
    let mut controller2 = controller1.clone();
    let actor = 9_742_820;
    let counter = 1;
    let dot = Dot::new(actor, counter);
    let op1 = controller1.txns.add(
        "member".to_owned(),
        controller1.txns.read().derive_add_ctx(actor),
    );

    let add_ctx = controller1.history_hashes.read_ctx().derive_add_ctx(actor);
    let op2 = controller1
        .history_hashes
        .update(10u64, add_ctx, |v, a| v.add(vec![8; 20], a));

    let op3 = controller1
        .candidates
        .add(vec![8; 20], controller1.txns.read().derive_add_ctx(actor));
    controller1.apply(Op {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
    });
    println!("controller1: {:#?}", controller1);

    let actor = 9_742_821;
    let counter = 1;
    let dot = Dot::new(actor, counter);
    let op1 = controller2.txns.add(
        "member".to_owned(),
        controller2.txns.read().derive_add_ctx(actor),
    );

    let add_ctx = controller2.history_hashes.read_ctx().derive_add_ctx(actor);
    let op2 = controller2
        .history_hashes
        .update(10u64, add_ctx, |v, a| v.add(vec![8; 20], a));

    let op3 = controller2
        .candidates
        .add(vec![8; 20], controller2.txns.read().derive_add_ctx(actor));
    controller2.apply(Op {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
    });
    println!("controller2: {:#?}", controller2);

    controller1.merge(controller2);
    println!("controller3: {:#?}", controller1);
}
