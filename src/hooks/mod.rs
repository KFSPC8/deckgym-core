/// These are the places/functions in the framework that custom logic is to be implemented per card.
/// That is those special "if Psyduck, do this", "if Darkrai, do that" kind of logic.
/// We call these "hooks" (like on_attach_tool, on_attach_energy, on_play, on_knockout, etc...).
mod core;
mod retreat;

pub(crate) use core::can_play_support;
pub(crate) use core::contains_energy;
pub(crate) use core::get_damage_from_attack;
pub(crate) use core::on_attach_tool;
pub(crate) use core::to_playable_card;
pub(crate) use retreat::can_retreat;
pub(crate) use retreat::get_retreat_cost;
