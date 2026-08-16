use std::collections::HashMap;
use crate::game::world::{BlockId, BlockStateId};
use crate::game::world::block::{BlockDefinition, BlockStateDefinition, BlockStateProperty};
use crate::registry::Identifier;

pub struct BlockRegistry {
    blocks: Vec<BlockDefinition>,
    states: Vec<BlockStateDefinition>,
    block_identifiers: HashMap<Identifier, BlockId>
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            states: vec![],
            block_identifiers: HashMap::new()
        }
    }

    pub fn block(&self, id: BlockId) -> Option<&BlockDefinition> {
        self.blocks.get(id.raw() as usize)
    }

    pub fn state(&self, state_id: BlockStateId) -> Option<&BlockStateDefinition> {
        self.states.get(state_id.raw() as usize)
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn state_count(&self) -> usize {
        self.states.len()
    }

    pub fn find_state(
        &self,
        block_id: BlockId,
        properties: &[BlockStateProperty]
    ) -> Option<&BlockStateDefinition> {
        let definition = self.blocks.get(block_id.raw() as usize)?;

        definition
            .states()
            .iter()
            .find_map(|state| {
                let state_definition = self.states.get(state.raw() as usize)?;
                if state_definition.properties() == properties {
                    Some(state_definition)
                } else {
                    None
                }
            })
    }

    pub fn register_block(
        &mut self,
        identifier: Identifier
    ) -> BlockId {
        if let Some(existing_id) = self.block_identifiers.get(&identifier) {
            return *existing_id;
        }

        let id = BlockId::new(self.blocks.len() as u32);
        let definition = BlockDefinition::new(id, identifier.clone(), vec![]);

        self.blocks.push(definition);
        self.block_identifiers.insert(identifier, id);
        id
    }

    pub fn register_state(
        &mut self,
        block_id: BlockId,
        properties: Vec<BlockStateProperty>,
        is_default: bool
    ) -> Option<BlockStateId> {
        let block = self.blocks.get_mut(block_id.raw() as usize)?;

        let state_id = BlockStateId::new(self.states.len() as u32);
        let definition = BlockStateDefinition::new(state_id, block_id, vec![]);

        self.states.push(definition);
        block.add_state(state_id);

        if is_default {
            block.set_default_state(state_id)
        }

        Some(state_id)
    }

    pub fn block_by_identifier(
        &self,
        identifier: &Identifier
    ) -> Option<&BlockDefinition> {
        let block_id = self.block_identifiers.get(identifier)?;
        self.blocks.get(block_id.raw() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_block_have_zero_id() {
        let mut registry = BlockRegistry::new();
        let id = registry
            .register_block(
                Identifier::parse("minecraft:stone").expect("can't create id")
            );

        assert_eq!(id, BlockId::new(0));
        assert_eq!(registry.block_count(), 1);
    }

    #[test]
    fn block_ids_follow_reg_order() {
        let mut registry = BlockRegistry::new();
        let id = registry
            .register_block(
                Identifier::parse("minecraft:stone").expect("can't create id")
            );

        let id2 = registry
            .register_block(
                Identifier::parse("minecraft:air").expect("can't create id")
            );

        assert_eq!(id, BlockId::new(0));
        assert_eq!(id2, BlockId::new(1));
        assert_eq!(registry.block_count(), 2);
    }
}