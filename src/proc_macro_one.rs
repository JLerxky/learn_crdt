use crdts::{GCounter, Orswot};
use crdts_macro::crdt;

#[crdt(u64)]
pub struct ControllerOne {
    candidates: Orswot<Vec<u8>, u64>,
    block_height: GCounter<u64>,
    pool: Pool,
}

#[crdt(u64)]
pub struct Pool {
    txns: Orswot<String, String>,
}

#[test]
fn test_cmrdt() {
    use crdts::CmRDT;
    let mut controller = ControllerOne::default();
    for actor in 1..=100 {
        for counter in 1..=3 {
            let dot = crdts::Dot::new(actor, counter);
            let txns_op = controller.pool.txns.add(
                format!("{actor}-{counter}"),
                controller
                    .pool
                    .txns
                    .read_ctx()
                    .derive_add_ctx(actor.to_string()),
            );
            let poor_op = PoolCrdtOp {
                dot,
                txns_op: Some(txns_op),
            };

            let candidates_op = controller.candidates.add(
                vec![actor as u8; 20],
                controller.candidates.read_ctx().derive_add_ctx(actor),
            );

            let block_height_op = controller.block_height.inc(actor);
            let controller_op = ControllerOneCrdtOp {
                dot,
                candidates_op: Some(candidates_op),
                block_height_op: Some(block_height_op),
                pool_op: Some(poor_op),
            };
            let controller_op_bytes = bincode::serialize(&controller_op).unwrap();
            println!("controller_op_bytes len: {}", controller_op_bytes.len());
            let controller_op = bincode::deserialize(&controller_op_bytes).unwrap();
            controller.apply(controller_op);
        }
    }
    println!("txns num: {:#?}", controller.pool.txns.read().val.len());
    println!("block_height: {:#?}", controller.block_height.read());
}

#[test]
fn test_cvrdt() {
    use crdts::CmRDT;
    use crdts::CvRDT;
    use crdts::Dot;
    let mut controller1 = ControllerOne::default();
    let mut controller2 = controller1.clone();
    for actor in 1..=300 {
        let counter = 1;
        let dot = Dot::new(actor, counter);
        let txns_op = controller1.pool.txns.add(
            format!("{actor}-{counter}"),
            controller1
                .pool
                .txns
                .read_ctx()
                .derive_add_ctx(actor.to_string()),
        );
        let poor_op = PoolCrdtOp {
            dot,
            txns_op: Some(txns_op),
        };

        let candidates_op = controller1.candidates.add(
            vec![actor as u8; 20],
            controller1.candidates.read_ctx().derive_add_ctx(actor),
        );

        let block_height_op = controller1.block_height.inc(actor);
        controller1.apply(ControllerOneCrdtOp {
            dot,
            candidates_op: Some(candidates_op),
            block_height_op: Some(block_height_op),
            pool_op: Some(poor_op),
        });
        // println!("controller1: {:#?}", controller1);

        let dot = Dot::new(actor, counter);
        let txns_op = controller2.pool.txns.add(
            format!("{actor}-{counter}"),
            controller2
                .pool
                .txns
                .read_ctx()
                .derive_add_ctx(actor.to_string()),
        );
        let poor_op = PoolCrdtOp {
            dot,
            txns_op: Some(txns_op),
        };

        let candidates_op = controller2.candidates.add(
            vec![actor as u8; 20],
            controller2.candidates.read_ctx().derive_add_ctx(actor),
        );

        let block_height_op = controller2.block_height.inc(actor);
        controller2.apply(ControllerOneCrdtOp {
            dot,
            candidates_op: Some(candidates_op),
            block_height_op: Some(block_height_op),
            pool_op: Some(poor_op),
        });
        // println!("controller2: {:#?}", controller2);

        let controller2 = bincode::serialize(&controller2).unwrap();
        println!("controller_bytes len: {}", controller2.len());
        let controller2 = bincode::deserialize(&controller2).unwrap();

        controller1.merge(controller2);
    }
    // println!("controller3: {:#?}", controller1);
}
