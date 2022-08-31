//! A "Cell" represents a DNA/AgentId pair - a space where one dna/agent
//! can track its source chain and service network requests / responses.

use holo_hash::AgentPubKey;
use holo_hash::DnaHash;
use holochain_serialized_bytes::prelude::*;
use std::fmt;

use crate::AppRoleId;

/// The unique identifier for a Cell.
/// Cells are uniquely determined by this pair - this pair is necessary
/// and sufficient to refer to a cell in a conductor
#[derive(
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    SerializedBytes,
    Ord,
    PartialOrd,
)]
pub struct CellId(DnaHash, AgentPubKey);

/// Delimiter in a clone id that separates the base cell's role id from the
/// clone index.
pub const CLONE_ID_DELIMITER: &str = ".";

/// Identifier of a clone cell, composed of the DNA's role id and the index
/// of the clone, starting at 0.
///
/// Example: `profiles.0`
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloneId(String);

impl CloneId {
    /// Construct a clone id from role id and clone index.
    pub fn new(role_id: &AppRoleId, clone_index: u32) -> Self {
        CloneId(format!("{}{}{}", role_id, CLONE_ID_DELIMITER, clone_index))
    }

    /// Get the clone's base cell's role id.
    pub fn as_base_role_id(&self) -> AppRoleId {
        let (role_id, _) = self.0.split_once(CLONE_ID_DELIMITER).unwrap();
        role_id.into()
    }

    /// Get the index of the clone cell.
    pub fn as_clone_index(&self) -> u32 {
        let (_, clone_index) = self.0.split_once(CLONE_ID_DELIMITER).unwrap();
        clone_index.parse::<u32>().unwrap()
    }

    /// Get an app role id representation of the clone id.
    pub fn as_app_role_id(&self) -> AppRoleId {
        self.0.clone()
    }
}

impl fmt::Display for CloneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.as_base_role_id(),
            CLONE_ID_DELIMITER,
            self.as_clone_index()
        )
    }
}

impl fmt::Display for CellId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell({}, {})", self.dna_hash(), self.agent_pubkey())
    }
}

impl CellId {
    /// Create a CellId from its components
    pub fn new(dna_hash: DnaHash, agent_pubkey: AgentPubKey) -> Self {
        CellId(dna_hash, agent_pubkey)
    }

    /// The dna hash/address for this cell.
    pub fn dna_hash(&self) -> &DnaHash {
        &self.0
    }

    /// The agent id / public key for this cell.
    pub fn agent_pubkey(&self) -> &AgentPubKey {
        &self.1
    }

    /// Into [DnaHash] and [AgentPubKey]
    pub fn into_dna_and_agent(self) -> (DnaHash, AgentPubKey) {
        (self.0, self.1)
    }
}

impl From<(DnaHash, AgentPubKey)> for CellId {
    fn from(pair: (DnaHash, AgentPubKey)) -> Self {
        Self(pair.0, pair.1)
    }
}
