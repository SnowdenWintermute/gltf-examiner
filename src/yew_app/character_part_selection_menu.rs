use crate::frontend_common::CharacterPartCategories;
use crate::yew_app::battle_combatant_spawner::BattleCombatantSpawner;
use crate::yew_app::character_selection_menu::CharacterSelectionMenu;
use crate::yew_app::execute_attack_sequence_button::ExecuteAttackSequenceButton;
use crate::yew_app::select_animation_button::SelectAnimationButton;
use crate::yew_app::select_character_part_button::SelectCharacterPartButton;
use crate::yew_app::store::AppStore;
use gloo::console::log;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CharacterPartSelectionMenu)]
pub fn character_part_selection_menu() -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();
    let show_ui_state = use_state(|| true);
    let cloned_show_ui_state = show_ui_state.clone();
    let handle_toggle_ui_click =
        Callback::from(move |_| cloned_show_ui_state.set(!*cloned_show_ui_state));

    let cloned_dispatch = dispatch.clone();
    let handle_select_character_click = Callback::from(move |event: MouseEvent| {
        let target_element = event.target().unwrap();
        let input = target_element.unchecked_into::<HtmlSelectElement>();
        log!(format!("seleted: {:?}", input.value()));
        cloned_dispatch.reduce_mut(|store| {
            store.selected_character_id = input.value().parse().expect("a number")
        })
    });

    let cloned_dispatch = dispatch.clone();
    let handle_select_target_click = Callback::from(move |event: MouseEvent| {
        let target_element = event.target().unwrap();
        let input = target_element.unchecked_into::<HtmlSelectElement>();
        log!(format!("targeted: {:?}", input.value()));
        cloned_dispatch
            .reduce_mut(|store| store.selected_target_id = input.value().parse().expect("a number"))
    });

    html!(
        <section class="p-2 w-fit max-w-full mb-1" >
        <button
            class="border border-slate-400 bg-slate-700 p-2 mb-1 pointer-events-auto"
            onclick={handle_toggle_ui_click}>
                {"Toggle UI Visibility"}
        </button>
        if *show_ui_state {
            <div class="pointer-events-auto">
                <div>
                    <h3 class="text-xl mb-1">{"Select Character ID"}</h3>
                    <CharacterSelectionMenu
                        handle_click={handle_select_character_click.clone()}
                        selected_id={app_state.selected_character_id}
                    />
                    <h3 class="text-xl mb-1">{"Select Target ID"}</h3>
                    <CharacterSelectionMenu
                        handle_click={handle_select_target_click.clone()}
                        selected_id={app_state.selected_target_id}
                    />
                    <BattleCombatantSpawner />
                </div>
                <ExecuteAttackSequenceButton />
            </div>

            <h3 class="text-xl mb-1">{"Animations"}</h3>
            <ul class="flex pointer-events-auto mb-2 flex-wrap w-[300px]">
                {app_state.animation_names.iter().map(|item| html!(<SelectAnimationButton name={item.clone()} />)).collect::<Html>()}
            </ul>

            <CharacterPartSelectionCategoryButtonGroup
                title={AttrValue::from("Heads")}
                parts={app_state.parts_available.heads.clone()}
                category={CharacterPartCategories::Head}
            />
            <CharacterPartSelectionCategoryButtonGroup
                title={AttrValue::from("Torsos")}
                parts={app_state.parts_available.torsos.clone()}
                category={CharacterPartCategories::Torso}
            />
            <CharacterPartSelectionCategoryButtonGroup
                title={AttrValue::from("Legs")}
                parts={app_state.parts_available.legs.clone()}
                category={CharacterPartCategories::Leg}
            />
            <CharacterPartSelectionCategoryButtonGroup
                title={AttrValue::from("Weapons")}
                parts={app_state.parts_available.weapons.clone()}
                category={CharacterPartCategories::Weapon}
            />
        }
        </section>
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    category: CharacterPartCategories,
    parts: HashSet<String>,
    title: AttrValue,
}

#[function_component(CharacterPartSelectionCategoryButtonGroup)]
pub fn character_part_selection_category_button_group(props: &Props) -> Html {
    html!(
    <div class="mb-2 w-fit">
        <h3 class="text-xl mb-1">{props.title.clone()}</h3>
        <ul class="flex pointer-events-auto">
        {props.parts.iter()
            .map(|item| html!(
                <li>
                    <SelectCharacterPartButton
                        name={item.clone()}
                        category={props.category.clone()}
                    />
                </li>
            ))
            .collect::<Html>()}
        </ul>
    </div>
    )
}
