use crate::game::world::{BlockId, BlockStateId};
use crate::registry::Identifier;

#[derive(Debug, Clone)]
pub struct BlockDefinition {
    id: BlockId,
    identifier: Identifier,
    states: Vec<BlockStateId>,
    default_state: Option<BlockStateId>
}

#[derive(Debug, Clone)]
pub struct BlockStateDefinition {
    state_id: BlockStateId,
    block_id: BlockId,
    properties: Vec<BlockStateProperty>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStateProperty {
    name: String,
    value: String,
}

impl BlockDefinition {
    pub fn new(id: BlockId, identifier: Identifier, states: Vec<BlockStateId>) -> Self {
        Self {
            id,
            identifier,
            states,
            default_state: None
        }
    }

    pub fn id(&self) -> BlockId {
        self.id
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn states(&self) -> &[BlockStateId] {
        &self.states
    }

    pub fn default_state(&self) -> Option<BlockStateId> {
        self.default_state
    }

    pub(super) fn add_state(&mut self, state_id: BlockStateId) {
        self.states.push(state_id)
    }

    pub(super) fn set_default_state(&mut self, default_state: BlockStateId) {
        self.default_state = Some(default_state);
    }
}

impl BlockStateDefinition {
    pub fn new(state_id: BlockStateId, block_id: BlockId, properties: Vec<BlockStateProperty>) -> Self {
        Self {
            state_id,
            block_id,
            properties
        }
    }

    pub fn state_id(&self) -> BlockStateId {
        self.state_id
    }

    pub fn block_id(&self) -> BlockId {
        self.block_id
    }

    pub fn properties(&self) -> &[BlockStateProperty] {
        &self.properties
    }

    pub fn property(&self, name: &str) -> Option<&str> {
        self.properties
            .iter()
            .find(|property| property.name() == name)
            .map(|property| property.value())
    }
}

impl BlockStateProperty {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        Self {
            name,
            value
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}