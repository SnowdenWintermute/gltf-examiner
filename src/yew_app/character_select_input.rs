use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::use_store;

use crate::yew_app::store::AppStore;

#[function_component(CharacterSelectInput)]
pub fn character_select_input() -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();

    let cloned_dispatch = dispatch.clone();
    let handle_change = Callback::from(move |event: Event| {
        let target_element = event.target().unwrap();
        let input = target_element.unchecked_into::<HtmlSelectElement>();
        log!(format!("seleted: {:?}", input.value()));
        cloned_dispatch.reduce_mut(|store| {
            store.selected_character_id = input.value().parse().expect("a number")
        })
    });

    html!(
    <select
        class="h-10 border border-slate-400 bg-slate-700 text-zinc-300"
        onchange={handle_change} value={format!("{}", app_state.selected_character_id )}
        >
        {app_state.character_ids.iter().enumerate().map(|( id, item )| html!(<option value={format!("{}",id)}>{ *item }</option>)).collect::<Vec<Html>>()}
    </select>
    )
}
