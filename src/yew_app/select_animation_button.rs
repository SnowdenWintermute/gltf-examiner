use crate::{
    comm_channels::MessageFromYew, frontend_common::CharacterAnimationSelection,
    yew_app::store::AppStore,
};
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub name: AttrValue,
}

#[function_component(SelectAnimationButton)]
pub fn select_animation_button(props: &Props) -> Html {
    let (app_state, _) = use_store::<AppStore>();

    let name_to_send = props.name.to_string();
    let cloned_app_state = app_state.clone();
    let handle_click = Callback::from(move |_| {
        if let Some(transmitter) = &cloned_app_state.transmitter_option {
            transmitter
                .send(MessageFromYew::SelectAnimation(
                    CharacterAnimationSelection {
                        name: name_to_send.clone(),
                        character_id: cloned_app_state.selected_character_id,
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
