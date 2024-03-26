use crate::{comm_channels::MessageFromYew, yew_app::store::AppStore};
use yew::prelude::*;
use yewdux::use_store;

#[function_component(SpawnCharacterButton)]
pub fn spawn_character_button() -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();

    let cloned_app_state = app_state.clone();
    let handle_click = Callback::from(move |_| {
        if let Some(transmitter) = &cloned_app_state.transmitter_option {
            dispatch.reduce_mut(|store| {
                transmitter
                    .send(MessageFromYew::SpawnCharacter(store.next_character_id))
                    .expect("could not send event");
                store.character_ids.push(store.next_character_id);
                store.selected_character_id = cloned_app_state.next_character_id;
                store.next_character_id += 1;
            });
        }
    });

    html!(
    <button onclick={handle_click} class={format!("h-10 min-w-30 border border-slate-400 bg-slate-700")}>
        {format!("Spawn next character with id : {}", app_state.next_character_id)}
    </button>
    )
}
