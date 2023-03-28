use crdts::{CmRDT, GCounter, Map, Orswot};
use crdts_derive::{crdt, CRDT};

#[crdt]
#[derive(Default, Debug)]
pub struct ControllerOne {
    txns: Orswot<String, u64>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
    block_height: GCounter<u64>,
}

#[test]
fn test() {
    let mut controller = ControllerOne::default();
    let actor = 9_742_820;
    let counter = 1;
    let dot = crdts::Dot::new(actor, counter);
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
    controller.apply(ControllerOneCrdtOp {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
        block_height_op: None,
    });
    println!("{:#?}", controller);

    let actor = 9_742_820;
    let counter = 2;
    let dot = crdts::Dot::new(actor, counter);
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
    controller.apply(ControllerOneCrdtOp {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
        block_height_op: None,
    });
    println!("{:#?}", controller);
}
