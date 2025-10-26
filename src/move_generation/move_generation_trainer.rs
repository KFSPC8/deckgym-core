use crate::{
    actions::SimpleAction,
    card_ids::CardId,
    card_logic::can_rare_candy_evolve,
    hooks::{can_play_support, get_stage, is_ultra_beast},
    models::{Card, EnergyType, TrainerCard, TrainerType},
    tool_ids::ToolId,
    State,
};

/// Helper function to check if a trainer card can be played and return the appropriate action
fn can_play_trainer(_state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    Some(vec![SimpleAction::Play {
        trainer_card: trainer_card.clone(),
    }])
}

/// Helper function to return empty action vector
fn cannot_play_trainer() -> Option<Vec<SimpleAction>> {
    Some(vec![])
}

/// Generate possible actions for a trainer card.
pub fn generate_possible_trainer_actions(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    if state.turn_count == 0 {
        return cannot_play_trainer(); // No trainers on initial setup phase
    }
    if trainer_card.trainer_card_type == TrainerType::Supporter && !can_play_support(state) {
        return cannot_play_trainer(); // dont even check which type it is
    }

    trainer_move_generation_implementation(state, trainer_card)
}

/// Returns None instead of panicing if the trainer card is not implemented; this is so that the
/// WASM module can do "feature detection", and know if a card is implemented.
pub fn trainer_move_generation_implementation(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    // Pokemon tools can be played if there is a space in the mat for them.
    if trainer_card.trainer_card_type == TrainerType::Tool {
        return can_play_tool(state, trainer_card);
    }

    let trainer_id = CardId::from_card_id(trainer_card.id.as_str()).expect("CardId should exist");
    match trainer_id {
        // Complex cases: need to check specific conditions
        CardId::PA001Potion => can_play_potion(state, trainer_card),
        CardId::A1219Erika | CardId::A1266Erika | CardId::A4b328Erika | CardId::A4b329Erika => {
            can_play_erika(state, trainer_card)
        }
        CardId::A1220Misty | CardId::A1267Misty => can_play_misty(state, trainer_card),
        CardId::A2a072Irida | CardId::A2a087Irida | CardId::A4b330Irida | CardId::A4b331Irida => {
            can_play_irida(state, trainer_card)
        }
        CardId::A3155Lillie
        | CardId::A3197Lillie
        | CardId::A3209Lillie
        | CardId::A4b348Lillie
        | CardId::A4b349Lillie
        | CardId::A4b374Lillie => can_play_lillie(state, trainer_card),
        CardId::A1222Koga | CardId::A1269Koga => can_play_koga(state, trainer_card),
        CardId::A1225Sabrina
        | CardId::A1272Sabrina
        | CardId::A4b338Sabrina
        | CardId::A4b339Sabrina => can_play_sabrina(state, trainer_card),
        CardId::A2150Cyrus | CardId::A2190Cyrus | CardId::A4b326Cyrus | CardId::A4b327Cyrus => {
            can_play_cyrus(state, trainer_card)
        }
        CardId::A2155Mars | CardId::A2195Mars | CardId::A4b344Mars | CardId::A4b345Mars => {
            can_play_trainer(state, trainer_card)
        }
        CardId::A3144RareCandy
        | CardId::A4b314RareCandy
        | CardId::A4b315RareCandy
        | CardId::A4b379RareCandy => can_play_rare_candy(state, trainer_card),
        CardId::A2b070PokemonCenterLady | CardId::A2b089PokemonCenterLady => {
            can_play_pokemon_center_lady(state, trainer_card)
        }
        CardId::A4151ElementalSwitch
        | CardId::A4b310ElementalSwitch
        | CardId::A4b311ElementalSwitch => can_play_elemental_switch(state, trainer_card),
        CardId::A3a064Repel => can_play_repel(state, trainer_card),
        CardId::A2146PokemonCommunication
        | CardId::A4b316PokemonCommunication
        | CardId::A4b317PokemonCommunication => can_play_pokemon_communication(state, trainer_card),
        CardId::A3a067Gladion | CardId::A3a081Gladion => can_play_gladion(state, trainer_card),
        CardId::A3a069Lusamine
        | CardId::A3a083Lusamine
        | CardId::A4b350Lusamine
        | CardId::A4b351Lusamine
        | CardId::A4b375Lusamine => can_play_lusamine(state, trainer_card),
        CardId::A4157Lyra | CardId::A4197Lyra | CardId::A4b332Lyra | CardId::A4b333Lyra => {
            can_play_lyra(state, trainer_card)
        }
        // Simple cases: always can play
        CardId::A4158Silver
        | CardId::A4198Silver
        | CardId::A4b336Silver
        | CardId::A4b337Silver
        | CardId::PA002XSpeed
        | CardId::PA005PokeBall
        | CardId::A2b111PokeBall
        | CardId::PA006RedCard
        | CardId::PA007ProfessorsResearch
        | CardId::A4b373ProfessorsResearch
        | CardId::A1223Giovanni
        | CardId::A1270Giovanni
        | CardId::A4b334Giovanni
        | CardId::A4b335Giovanni
        | CardId::A1a065MythicalSlab
        | CardId::A1a068Leaf
        | CardId::A1a082Leaf
        | CardId::A4b346Leaf
        | CardId::A4b347Leaf
        | CardId::A2b071Red
        | CardId::A2b090Red
        | CardId::A4b352Red
        | CardId::A4b353Red => can_play_trainer(state, trainer_card),
        CardId::A3b066EeveeBag
        | CardId::A3b107EeveeBag
        | CardId::A4b308EeveeBag
        | CardId::A4b309EeveeBag => can_play_eevee_bag(state, trainer_card),
        _ => None,
    }
}

/// Check if a Pokemon tool can be played (requires at least 1 pokemon in play without a tool)
fn can_play_tool(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let &tool_id = ToolId::from_trainer_card(trainer_card).expect("ToolId should exist");

    let valid_targets = tool_id
        .enumerate_choices(state, state.current_player)
        .count();
    if valid_targets > 0 {
        Some(vec![SimpleAction::Play {
            trainer_card: trainer_card.clone(),
        }])
    } else {
        Some(vec![])
    }
}

/// Check if Potion can be played (requires at least 1 damaged pokemon in play)
fn can_play_potion(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged())
        .count();
    if damaged_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Erika can be played (requires at least 1 damaged Grass pokemon in play)
fn can_play_erika(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_grass_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged() && x.get_energy_type() == Some(EnergyType::Grass))
        .count();
    if damaged_grass_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Irida can be played (requires at least 1 damaged pokemon with Water energy attached)
fn can_play_irida(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_water_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged() && x.attached_energy.contains(&EnergyType::Water))
        .count();
    if damaged_water_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

fn can_play_elemental_switch(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    if state.maybe_get_active(state.current_player).is_none() {
        return cannot_play_trainer();
    }
    let allowed_types = [EnergyType::Fire, EnergyType::Water, EnergyType::Lightning];
    let has_valid_source =
        state
            .enumerate_bench_pokemon(state.current_player)
            .any(|(_, pokemon)| {
                pokemon
                    .attached_energy
                    .iter()
                    .any(|energy| allowed_types.contains(energy))
            });

    if has_valid_source {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Pokemon Center Lady can be played (requires at least 1 damaged or status-affected pokemon)
fn can_play_pokemon_center_lady(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let has_valid_target = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.is_damaged() || pokemon.has_status_condition());
    if has_valid_target {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Lillie can be played (requires at least 1 damaged Stage 2 pokemon in play)
fn can_play_lillie(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let damaged_stage2_count = state
        .enumerate_in_play_pokemon(state.current_player)
        .filter(|(_, x)| x.is_damaged() && get_stage(x) == 2)
        .count();
    if damaged_stage2_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Misty can be played (requires at least 1 water pokemon in play)
fn can_play_misty(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let water_in_player_count = state.num_in_play_of_type(state.current_player, EnergyType::Water);
    if water_in_player_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Koga can be played (requires active pokemon to be Weezing or Muk)
fn can_play_koga(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let active_pokemon = &state.maybe_get_active(state.current_player);
    if let Some(played_card) = active_pokemon {
        let card_id =
            CardId::from_card_id(played_card.get_id().as_str()).expect("CardId should be known");
        match card_id {
            CardId::A1177Weezing | CardId::A1243Weezing | CardId::A1175Muk => {
                return can_play_trainer(state, trainer_card);
            }
            _ => {}
        }
    }
    cannot_play_trainer()
}

/// Check if Sabrina can be played (requires opponent to have benched pokemon)
fn can_play_sabrina(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let opponent_has_bench = state.enumerate_bench_pokemon(opponent).count() > 0;
    if opponent_has_bench {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Cyrus can be played (requires opponent to have at least 1 damaged bench pokemon)
fn can_play_cyrus(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let damaged_bench_count = state
        .enumerate_bench_pokemon(opponent)
        .filter(|(_, x)| x.is_damaged())
        .count();
    if damaged_bench_count > 0 {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Repel can be played (requires opponent's active to be a Basic pokemon)
fn can_play_repel(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let opponent = (state.current_player + 1) % 2;
    let opponent_active = &state.maybe_get_active(opponent);
    let opponent_bench_count = state.enumerate_bench_pokemon(opponent).count();
    if let Some(opponent_active) = opponent_active {
        if opponent_active.card.is_basic() && opponent_bench_count > 0 {
            return can_play_trainer(state, trainer_card);
        }
    }
    cannot_play_trainer()
}

fn can_play_rare_candy(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    if state.is_users_first_turn() {
        return cannot_play_trainer();
    }

    let player = state.current_player;
    let hand = &state.hands[player];

    // Check if there's at least 1 basic pokemon in field with a corresponding stage2-rare-candy-evolvable in hand
    let has_valid_evolution_pair = state
        .enumerate_in_play_pokemon(player)
        .any(|(_, in_play)| hand.iter().any(|card| can_rare_candy_evolve(card, in_play)));
    if has_valid_evolution_pair {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Pokemon Communication can be played (requires at least 1 Pokemon in hand and 1 in deck)
fn can_play_pokemon_communication(
    state: &State,
    trainer_card: &TrainerCard,
) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let has_pokemon_in_hand = state.hands[player]
        .iter()
        .any(|card| matches!(card, Card::Pokemon(_)));
    let has_pokemon_in_deck = state.decks[player]
        .cards
        .iter()
        .any(|card| matches!(card, Card::Pokemon(_)));
    if has_pokemon_in_hand && has_pokemon_in_deck {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}

/// Check if Gladion can be played (requires possibility of Type: Null or Silvally in deck)
fn can_play_gladion(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;

    // Count Type: Null and Silvally in play and discard
    let mut type_null_count = 0;
    let mut silvally_count = 0;

    // Count in play Pokemon (including cards_behind)
    for pokemon in state.in_play_pokemon[player].iter().flatten() {
        // Check the current card
        if pokemon.get_name() == "Type: Null" {
            type_null_count += 1;
        } else if pokemon.get_name() == "Silvally" {
            silvally_count += 1;
        }

        // Check cards_behind (evolution chain)
        for card in &pokemon.cards_behind {
            if card.get_name() == "Type: Null" {
                type_null_count += 1;
            } else if card.get_name() == "Silvally" {
                silvally_count += 1;
            }
        }
    }

    // Count in discard pile
    for card in &state.discard_piles[player] {
        if card.get_name() == "Type: Null" {
            type_null_count += 1;
        } else if card.get_name() == "Silvally" {
            silvally_count += 1;
        }
    }

    // Can play if we haven't accounted for all 2 Type: Null and 2 Silvally
    // (meaning there might still be some in the deck)
    if type_null_count >= 2 && silvally_count >= 2 {
        cannot_play_trainer()
    } else {
        can_play_trainer(state, trainer_card)
    }
}

/// Check if Lusamine can be played (requires opponent has >= 1 point, player has Ultra Beast, >= 1 energy in discard)
fn can_play_lusamine(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let opponent = (player + 1) % 2;

    // Check if opponent has at least 1 point
    if state.points[opponent] < 1 {
        return cannot_play_trainer();
    }

    // Check if player has at least 1 Ultra Beast in play
    let has_ultra_beast = state.in_play_pokemon[player]
        .iter()
        .flatten()
        .any(|pokemon| is_ultra_beast(&pokemon.get_name()));
    if !has_ultra_beast {
        return cannot_play_trainer();
    }

    // Check if player has at least 1 energy in discard
    if state.discard_energies[player].is_empty() {
        return cannot_play_trainer();
    }

    can_play_trainer(state, trainer_card)
}

/// Check if Lyra can be played (requires active pokemon to have damage and at least 1 benched pokemon)
fn can_play_lyra(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let player = state.current_player;
    let active_pokemon = state.maybe_get_active(player);
    let bench_count = state.enumerate_bench_pokemon(player).count();

    if let Some(active) = active_pokemon {
        if active.is_damaged() && bench_count > 0 {
            return can_play_trainer(state, trainer_card);
        }
    }
    cannot_play_trainer()
}

/// Check if Eevee Bag can be played (requires at least 1 Pokemon that evolved from Eevee in play)
fn can_play_eevee_bag(state: &State, trainer_card: &TrainerCard) -> Option<Vec<SimpleAction>> {
    let has_eevee_evolution = state
        .enumerate_in_play_pokemon(state.current_player)
        .any(|(_, pokemon)| pokemon.evolved_from("Eevee"));
    if has_eevee_evolution {
        can_play_trainer(state, trainer_card)
    } else {
        cannot_play_trainer()
    }
}
