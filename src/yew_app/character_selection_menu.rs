use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::use_store;

use crate::yew_app::store::AppStore;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_click: Callback<MouseEvent>,
    pub selected_id: u32,
}

#[function_component(CharacterSelectionMenu)]
pub fn character_selection_menu(props: &Props) -> Html {
    let (app_state, _) = use_store::<AppStore>();

    html!(
    <ul
        class="flex border border-slate-400 bg-slate-700 text-zinc-300 w-fit"
        >
        {app_state.character_ids.iter().map(|id|
                {
                    let is_selected = *id == props.selected_id;
                    let conditional_styles = if is_selected {
                        "bg-slate-900"
                    } else {
                        ""
                    };

                    html!(
                        <button
                            onclick={props.handle_click.clone()}
                            value={format!("{id}")}
                            class={format!(  "border-r border-slate-400 last:border-r-0 p-2 {}", conditional_styles  )}
                            >{ *id }
                        </button>
                        )
                }
                ).collect::<Vec<Html>>()}
    </ul>
    )
}
