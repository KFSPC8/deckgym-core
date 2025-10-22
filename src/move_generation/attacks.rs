use crate::{
    actions::SimpleAction, attack_ids::AttackId, effects::CardEffect, hooks::contains_energy, State,
};

pub(crate) fn generate_attack_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    let mut actions = Vec::new();
    if let Some(active_pokemon) = &state.in_play_pokemon[current_player][0] {
        // Check if the active Pokémon has the CannotAttack effect
        let active_effects = active_pokemon.get_active_effects();
        let cannot_attack = active_effects
            .iter()
            .any(|effect| matches!(effect, CardEffect::CannotAttack));
        if cannot_attack {
            return actions;
        }

        let restricted_attacks: Vec<AttackId> = active_effects
            .iter()
            .filter_map(|effect| match effect {
                CardEffect::CannotUseAttack(attack_id) => Some(*attack_id),
                _ => None,
            })
            .collect();

        let pokemon_id = active_pokemon.get_id();
        for (i, attack) in active_pokemon.get_attacks().iter().enumerate() {
            if contains_energy(
                active_pokemon,
                &attack.energy_required,
                state,
                current_player,
            ) {
                let attack_is_restricted = AttackId::from_pokemon_index(&pokemon_id[..], i)
                    .map(|attack_id| restricted_attacks.contains(&attack_id))
                    .unwrap_or(false);
                if attack_is_restricted {
                    continue;
                }

                actions.push(SimpleAction::Attack(i));
            }
        }
    }
    actions
}
