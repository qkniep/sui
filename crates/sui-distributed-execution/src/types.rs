use std::collections::{HashSet, HashMap, BTreeMap};
use sui_protocol_config::ProtocolConfig;
use sui_types::{
    base_types::{ObjectID, ObjectRef, SequenceNumber},
    digests::TransactionDigest,
    epoch_data::EpochData,
    messages::{InputObjectKind, VerifiedTransaction, TransactionKind, TransactionDataAPI},
    object::Object,
    sui_system_state::epoch_start_sui_system_state::EpochStartSystemState,
    effects::TransactionEffects, storage::{DeleteKind, WriteKind},
};


#[derive(Debug)]
pub enum SailfishMessage {
    // Sequencing Worker <-> Execution Worker
    EpochStart { conf: ProtocolConfig, data: EpochData, ref_gas_price: u64 },
    EpochEnd { new_epoch_start_state: EpochStartSystemState },
    ProposeExec(Transaction),

    // Execution Worker <-> Execution Worker
    //LockedExec { tx: TransactionDigest, objects: Vec<(ObjectRef, Object)> },
    LockedExec { txid: TransactionDigest, objects: Vec<Option<(ObjectRef, Object)>>, child_objects: Vec<Option<(ObjectRef, Object)>> },
    MissingObjects { txid: TransactionDigest, ew: u8, missing_objects: HashSet<ObjectID> },
    TxResults { txid: TransactionDigest, deleted: BTreeMap<ObjectID, (SequenceNumber, DeleteKind)>, written: BTreeMap<ObjectID, (ObjectRef, Object, WriteKind)> },

    // Execution Worker <-> Storage Engine
    StateUpdate(TransactionEffects),
    Checkpointed(u64),
}


#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx: VerifiedTransaction,
    pub ground_truth_effects: TransactionEffects,  // full effects of tx, as ground truth exec result
    pub child_inputs: Vec<ObjectID>, // TODO: mark mutable
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

        for (gas_obj_id, _, _) in tx_data.gas().iter() {
            // skip genesis gas objects
            if *gas_obj_id != ObjectID::from_single_byte(0) {
                read_set.insert(*gas_obj_id);
            }
        }

        for (&package_obj_id, _, _) in tx_data.move_calls() {
            read_set.insert(package_obj_id);
        }

        return read_set;
    }

    /// TODO: This makes use of ground_truth_effects, which is illegal for validators;
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

    pub fn get_relevant_ews(&self, num_ews: u8) -> HashSet<u8> {
        let rw_set = self.get_read_write_set();
        if rw_set.contains(&ObjectID::from_single_byte(5)) || self.is_epoch_change() {
            (0..num_ews).collect()
        } else {
            rw_set.into_iter().map(|obj_id| obj_id[0] % num_ews).collect()
        }
    }
}

pub struct TransactionWithResults {
    pub full_tx: Transaction,
    pub tx_effects: TransactionEffects,            // determined after execution
    pub deleted: BTreeMap<ObjectID, (SequenceNumber, DeleteKind)>,
    pub written: BTreeMap<ObjectID, (ObjectRef, Object, WriteKind)>,
    pub missing_objs: HashSet<ObjectID>,
}
