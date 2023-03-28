use crdts::{CmRDT, GCounter, Map, Orswot};
use crdts_derive::crdt;

#[crdt(u64)]
#[derive(Default, Debug)]
pub struct ControllerOne {
    txns: Orswot<String, String>,
    history_hashes: Map<u64, Orswot<Vec<u8>, u64>, u64>,
    candidates: Orswot<Vec<u8>, u64>,
    block_height: GCounter<u64>,
}

#[test]
fn test() {
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
