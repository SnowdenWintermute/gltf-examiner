use crate::{
    comm_channels::MessageFromYew,
    frontend_common::{CharacterPartCategories, CharacterPartSelection},
    yew_app::store::AppStore,
};
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub name: AttrValue,
    pub category: CharacterPartCategories,
}

#[function_component(SelectCharacterPartButton)]
pub fn select_character_part_button(props: &Props) -> Html {
    let (app_state, _) = use_store::<AppStore>();

    let name_to_send = props.name.to_string();
    let cloned_app_state = app_state.clone();
    let category = props.category.clone();
    let handle_click = Callback::from(move |_| {
        if let Some(transmitter) = &cloned_app_state.transmitter_option {
            transmitter
                .send(MessageFromYew::SelectCharacterPart(
                    CharacterPartSelection {
                        character_id: app_state.selected_character_id,
                        name: name_to_send.clone(),
                        category: category.clone(),
                    },
                ))
                .expect("could not send event");
        }
    });

    html!(
    <button onclick={handle_click} class={format!("h-10 min-w-30 border border-slate-400 bg-slate-700 border-r-0 last:border-r pr-2 pl-2")}>
      {props.name.clone()}
    </button>
    )
}
