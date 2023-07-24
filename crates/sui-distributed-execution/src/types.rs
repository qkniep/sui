use std::collections::HashSet;
use sui_protocol_config::ProtocolConfig;
use sui_types::{
    base_types::{ObjectID, ObjectRef},
    digests::TransactionDigest,
    epoch_data::EpochData,
    messages::{InputObjectKind, VerifiedTransaction, TransactionKind, TransactionDataAPI},
    object::Object,
    sui_system_state::epoch_start_sui_system_state::EpochStartSystemState,
    effects::TransactionEffects,
};


#[derive(Debug)]
pub enum SailfishMessage {
    // Sequencing Worker <-> Execution Worker
    EpochStart { conf: ProtocolConfig, data: EpochData, ref_gas_price: u64 },
    EpochEnd { new_epoch_start_state: EpochStartSystemState },
    ProposeExec(Transaction),

    // Execution Worker <-> Execution Worker
    LockedExec { tx: TransactionDigest, objects: Vec<(ObjectRef, Object)> },

    // Execution Worker <-> Storage Engine
    StateUpdate(TransactionEffects),
    Checkpointed(u64),
}


#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx: VerifiedTransaction,
    pub ground_truth_effects: TransactionEffects,  // full effects of tx, as ground truth exec result
    pub checkpoint_seq: u64,
}

impl Transaction {
    pub fn is_epoch_change(&self) -> bool {
        match self.tx.data().transaction_data().kind() {
            TransactionKind::ChangeEpoch(_) => true,
            _ => false,
        }
    }

    /// Returns the read set of a transction.
    /// Specifically, this is the set of input objects to the transaction.
    /// It excludes child objects that are determined at runtime,
    /// but includes all owned objects inputs that must have their version numbers bumped.
    pub fn get_read_set(&self) -> HashSet<ObjectID> {
        let tx_data = self.tx.data().transaction_data();
        let input_object_kinds = tx_data
            .input_objects()
            .expect("Cannot get input object kinds");
    
        let mut read_set = HashSet::new();
        for kind in &input_object_kinds {
            match kind {
                InputObjectKind::MovePackage(id)
                | InputObjectKind::SharedMoveObject { id, .. }
                | InputObjectKind::ImmOrOwnedMoveObject((id, _, _)) => {
                    read_set.insert(*id)
                }
            };
        }
        return read_set;
    }

    /// TODO: This makes use of ground_truth_effects, which is illegal;
    /// it is not something that is known a-priori before execution.
    /// Returns the write set of a transction.
    pub fn get_write_set(&self) -> HashSet<ObjectID> {
        let TransactionEffects::V1(tx_effects) = &self.ground_truth_effects;
        let total_writes = tx_effects.created.len() + tx_effects.mutated.len() + tx_effects.unwrapped.len() + tx_effects.deleted.len() + tx_effects.unwrapped_then_deleted.len() + tx_effects.wrapped.len();
        let mut write_set: HashSet<ObjectID> = HashSet::with_capacity(total_writes);

        write_set.extend(tx_effects.created
            .iter()
            .chain(tx_effects.mutated.iter())
            .chain(tx_effects.unwrapped.iter())
            .map(|(object_ref, _)| object_ref.0)
        );
        write_set.extend(tx_effects.deleted
            .iter()
            .chain(tx_effects.unwrapped_then_deleted.iter())
            .chain(tx_effects.wrapped.iter())
            .map(|object_ref| object_ref.0));

        return write_set;
    }

    /// Returns the read-write set of the transaction.
    pub fn get_read_write_set(&self) -> HashSet<ObjectID> {
        self.get_read_set()
            .union(&self.get_write_set())
            .copied()
            .collect()
    }
}

pub struct TransactionWithResults {
    pub full_tx: Transaction,
    pub tx_effects: TransactionEffects,            // determined after execution
}
