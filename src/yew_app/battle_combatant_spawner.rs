use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::comm_channels::MessageFromYew;
use crate::frontend_common::CombatantSpecies;
use crate::yew_app::store::AppStore;
use bevy::transform::components::Transform;
use gloo::console::info;
use std::f32::consts::PI;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(BattleCombatantSpawner)]
pub fn battle_combatant_spawner() -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();
    let bevy_assets_loaded = app_state.bevy_assets_loaded;

    // info!(format!("assets loaded {}", bevy_assets_loaded));
    let cloned_app_state = app_state.clone();
    let cloned_dispatch = dispatch.clone();
    use_effect_with(bevy_assets_loaded, move |_| {
        if !bevy_assets_loaded || cloned_app_state.character_ids.len() > 0 {
            return;
        }
        // info!("useffect detected assets loaded");
        if let Some(transmitter) = &cloned_app_state.transmitter_option {
            let mut home_location = HomeLocation(Transform::from_xyz(0.0, 0.0, -1.5));
            home_location.0.rotate_y(PI);
            for _ in 0..=2 as u32 {
                cloned_dispatch.reduce_mut(|store| {
                    transmitter
                        .send(MessageFromYew::SpawnCharacterWithHomeLocation(
                            store.next_character_id,
                            home_location.clone(),
                            CombatantSpecies::Humanoid,
                        ))
                        .expect("could not send event");
                    store.next_character_id += 1
                });
                home_location.0.translation.x += 1.0;
            }

            let mut home_location = HomeLocation(Transform::from_xyz(0.0, 0.0, 1.5));

            for _ in 3..=5 as u32 {
                cloned_dispatch.reduce_mut(|store| {
                    transmitter
                        .send(MessageFromYew::SpawnCharacterWithHomeLocation(
                            store.next_character_id,
                            home_location.clone(),
                            CombatantSpecies::Spider,
                        ))
                        .expect("could not send event");
                    store.next_character_id += 1
                });
                home_location.0.translation.x += 1.0;
            }
        }
    });

    html!()
}
