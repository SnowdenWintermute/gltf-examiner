use crate::{
    frontend_common::CharacterPartCategories,
    yew_app::{
        character_select_input::CharacterSelectInput,
        execute_attack_sequence_button::ExecuteAttackSequenceButton,
        select_animation_button::SelectAnimationButton,
        select_character_part_button::SelectCharacterPartButton,
        spawn_character_button::SpawnCharacterButton, store::AppStore,
    },
};
use std::collections::HashSet;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CharacterPartSelectionMenu)]
pub fn character_part_selection_menu() -> Html {
    let (app_state, _) = use_store::<AppStore>();
    let show_ui_state = use_state(|| true);
    let cloned_show_ui_state = show_ui_state.clone();
    let handle_toggle_ui_click =
        Callback::from(move |_| cloned_show_ui_state.set(!*cloned_show_ui_state));

    html!(
        <section class="p-2 w-fit max-w-full mb-1" >
        <button
            class="border border-slate-400 bg-slate-700 p-2 mb-1 pointer-events-auto"
            onclick={handle_toggle_ui_click}>
                {"Toggle UI Visibility"}
        </button>
        if *show_ui_state {
            <div class="pointer-events-auto">
                <h3 class="text-xl mb-1">{"Select Character ID"}</h3>
                <div>
                    <CharacterSelectInput />
                    <SpawnCharacterButton />
                </div>
                <ExecuteAttackSequenceButton />
            </div>

            <h3 class="text-xl mb-1">{"Animations"}</h3>
            <ul class="flex pointer-events-auto mb-2 flex-wrap">
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
