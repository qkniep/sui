// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    account_address::AccountAddress,
    ident_str,
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
};
use serde::{Deserialize, Serialize};

use crate::{
    base_types::{ObjectID, SequenceNumber},
    id::ID,
    SUI_FRAMEWORK_ADDRESS,
};

const TRANSFER_MODULE_NAME: &IdentStr = ident_str!("transfer");
const INBOX_REFERENCE_STRUCT_NAME: &IdentStr = ident_str!("Receiving");

pub const RESOLVED_RECEIVING_STRUCT: (&AccountAddress, &IdentStr, &IdentStr) = (
    &SUI_FRAMEWORK_ADDRESS,
    TRANSFER_MODULE_NAME,
    INBOX_REFERENCE_STRUCT_NAME,
);

/// Rust version of the Move sui::transfer::Receiving type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Receiving {
    pub id: ID,
    pub version: SequenceNumber,
}

impl Receiving {
    pub fn new(id: ObjectID, version: SequenceNumber) -> Self {
        Self {
            id: ID::new(id),
            version,
        }
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(self).expect("Value representation is owned and should always serialize")
    }

    pub fn struct_tag() -> StructTag {
        StructTag {
            address: SUI_FRAMEWORK_ADDRESS,
            module: TRANSFER_MODULE_NAME.to_owned(),
            name: INBOX_REFERENCE_STRUCT_NAME.to_owned(),
            // TODO(tzakian): type param here is needed?
            type_params: vec![],
        }
    }

    pub fn type_tag() -> TypeTag {
        TypeTag::Struct(Box::new(Self::struct_tag()))
    }
}
