use crdts::{GCounter, Map, Orswot};
use crdts_derive::crdt;

#[crdt(u64)]
#[derive(Default, Debug, Clone)]
pub struct ControllerOne {
    txns: Orswot<String, String>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
    block_height: GCounter<u64>,
}

#[test]
fn test_cmrdt() {
    use crdts::CmRDT;
    let mut controller = ControllerOne::default();
    for actor in 1..=100 {
        for counter in 1..=3 {
            let dot = crdts::Dot::new(actor, counter);
            let op1 = controller.txns.add(
                format!("{actor}-{counter}"),
                controller.txns.read().derive_add_ctx(actor.to_string()),
            );

            let add_ctx = controller.history_hashes.read_ctx().derive_add_ctx(actor);
            let op2 = controller
                .history_hashes
                .update(actor, add_ctx, |v, a| v.add(vec![actor as u8; 20], a));

            let op3 = controller.candidates.add(
                vec![actor as u8; 20],
                controller.candidates.read().derive_add_ctx(actor),
            );

            let op4 = controller.block_height.inc(actor);
            controller.apply(ControllerOneCrdtOp {
                dot,
                txns_op: Some(op1),
                history_hashes_op: Some(op2),
                candidates_op: Some(op3),
                block_height_op: Some(op4),
            });
        }
    }
    println!("txns num: {:#?}", controller.txns.read().val.len());
    println!("block_height: {:#?}", controller.block_height.read());
}

#[test]
fn test_cvrdt() {
    use crdts::CmRDT;
    use crdts::CvRDT;
    use crdts::Dot;
    let mut controller1 = ControllerOne::default();
    let mut controller2 = controller1.clone();
    let actor = 9_742_820;
    let counter = 1;
    let dot = Dot::new(actor, counter);
    let op1 = controller1.txns.add(
        format!("{actor}-{counter}"),
        controller1.txns.read().derive_add_ctx(actor.to_string()),
    );

    let add_ctx = controller1.history_hashes.read_ctx().derive_add_ctx(actor);
    let op2 = controller1
        .history_hashes
        .update(actor, add_ctx, |v, a| v.add(vec![actor as u8; 20], a));

    let op3 = controller1.candidates.add(
        vec![actor as u8; 20],
        controller1.candidates.read().derive_add_ctx(actor),
    );

    let op4 = controller1.block_height.inc(actor);
    controller1.apply(ControllerOneCrdtOp {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
        block_height_op: Some(op4),
    });
    println!("controller1: {:#?}", controller1);

    let actor = 9_742_821;
    let counter = 1;
    let dot = Dot::new(actor, counter);
    let op1 = controller2.txns.add(
        format!("{actor}-{counter}"),
        controller2.txns.read().derive_add_ctx(actor.to_string()),
    );

    let add_ctx = controller2.history_hashes.read_ctx().derive_add_ctx(actor);
    let op2 = controller2
        .history_hashes
        .update(actor, add_ctx, |v, a| v.add(vec![actor as u8; 20], a));

    let op3 = controller2.candidates.add(
        vec![actor as u8; 20],
        controller2.candidates.read().derive_add_ctx(actor),
    );

    let op4 = controller2.block_height.inc(actor);
    controller2.apply(ControllerOneCrdtOp {
        dot,
        txns_op: Some(op1),
        history_hashes_op: Some(op2),
        candidates_op: Some(op3),
        block_height_op: Some(op4),
    });
    println!("controller2: {:#?}", controller2);

    controller1.merge(controller2);
    println!("controller3: {:#?}", controller1);
}
