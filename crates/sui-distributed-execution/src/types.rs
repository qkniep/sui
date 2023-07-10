use std::collections::HashSet;
use sui_protocol_config::ProtocolConfig;
use sui_types::{
    base_types::ObjectID,
    epoch_data::EpochData,
    messages::{InputObjectKind, VerifiedTransaction, TransactionKind, TransactionDataAPI},
    sui_system_state::epoch_start_sui_system_state::EpochStartSystemState,
    effects::{TransactionEffects},
};


#[derive(Debug, Clone)]
pub enum SailfishMessage {
    EpochStart{conf: ProtocolConfig, data: EpochData, ref_gas_price: u64},
    EpochEnd{new_epoch_start_state: EpochStartSystemState},
    Transaction{tx: VerifiedTransaction, tx_effects: TransactionEffects, checkpoint_seq: u64}
}


#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx: VerifiedTransaction,
    pub ground_truth_effects: TransactionEffects,  // full effects of tx, as ground truth exec result
    pub checkpoint_seq: u64,
}

impl Transaction {
    pub fn is_epoch_change(&self) -> bool {
        if let TransactionKind::ChangeEpoch(_) = self.tx.data().transaction_data().kind() {
            return true;
        }
        return false;
    }

    /// Returns the read set of a transction
    /// Specifically, this is the set of input objects to the transaction. It excludes 
    /// child objects that are determined at runtime, but includes all owned objects inputs
    /// that must have their version numbers bumped.
    fn get_read_set(&self) -> HashSet<ObjectID> {
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

    /// TODO: This makes use of ground_truth_effects, which is illegal; it is not something that is 
    /// known a-priori before execution
    /// Returns the write set of a transction
    fn get_write_set(&self) -> HashSet<ObjectID> {

        let mut write_set: HashSet<ObjectID> = HashSet::new();

        let TransactionEffects::V1(tx_effects) = &self.ground_truth_effects;

        let created: Vec<ObjectID> = tx_effects.created.clone()
            .into_iter()
            .map(|(object_ref, _)| object_ref.0)
            .collect();
        let mutated: Vec<ObjectID> = tx_effects.mutated.clone()
            .into_iter()
            .map(|(object_ref, _)| object_ref.0)
            .collect();
        let unwrapped: Vec<ObjectID> = tx_effects.unwrapped.clone()
            .into_iter()
            .map(|(object_ref, _)| object_ref.0)
            .collect();
        let deleted: Vec<ObjectID> = tx_effects.deleted.clone()
            .into_iter()
            .map(|object_ref| object_ref.0)
            .collect();
        let unwrapped_then_deleted: Vec<ObjectID> = tx_effects.unwrapped_then_deleted.clone()
            .into_iter()
            .map(|object_ref| object_ref.0)
            .collect();
        let wrapped: Vec<ObjectID> = tx_effects.wrapped.clone()
            .into_iter()
            .map(|object_ref| object_ref.0)
            .collect();

        write_set.extend(created);
        write_set.extend(mutated);
        write_set.extend(unwrapped);
        write_set.extend(deleted);
        write_set.extend(unwrapped_then_deleted);
        write_set.extend(wrapped);
        return write_set;
    }

    /// Returns the read-write set of the transaction
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
