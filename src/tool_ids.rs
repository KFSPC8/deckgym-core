use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    models::{EnergyType, PlayedCard, TrainerCard},
    State,
};

// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolId {
    A2147GiantCape,
    A2148RockyHelmet,
    A3147LeafCape,
    A4a067InflatableBoat,
}

lazy_static::lazy_static! {
    static ref TOOL_ID_MAP: HashMap<&'static str, ToolId> = {
        let mut m = HashMap::new();
        m.insert("A2 147", ToolId::A2147GiantCape);
        m.insert("A2 148", ToolId::A2148RockyHelmet);
        m.insert("A3 147", ToolId::A3147LeafCape);
        m.insert("A4a 067", ToolId::A4a067InflatableBoat);
        m
    };
}

impl ToolId {
    pub fn from_trainer_card(trainer_card: &TrainerCard) -> Option<&Self> {
        TOOL_ID_MAP.get(&trainer_card.id.as_str())
    }

    /// Check if a tool can be attached to a specific pokemon
    pub fn can_attach_to(&self, pokemon: &PlayedCard) -> bool {
        match self {
            ToolId::A3147LeafCape => {
                // Leaf Cape can only be attached to Grass pokemon
                pokemon.card.get_type() == Some(EnergyType::Grass)
            }
            ToolId::A4a067InflatableBoat => {
                // Inflatable Boat can only be attached to Water pokemon
                pokemon.card.get_type() == Some(EnergyType::Water)
            }
            // Most tools can be attached to any pokemon
            ToolId::A2147GiantCape | ToolId::A2148RockyHelmet => true,
        }
    }

    pub(crate) fn enumerate_choices<'a>(
        &self,
        state: &'a State,
        actor: usize,
    ) -> impl Iterator<Item = (usize, &'a PlayedCard)> {
        let tool_id = *self;
        state
            .enumerate_in_play_pokemon(actor)
            .filter(|(_, x)| !x.has_tool_attached())
            .filter(move |(_, x)| tool_id.can_attach_to(x))
    }
}
