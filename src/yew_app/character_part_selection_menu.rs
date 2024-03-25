use crate::{
    frontend_common::CharacterPartCategories,
    yew_app::{
        character_select_input::CharacterSelectInput,
        select_character_part_button::SelectCharacterPartButton,
        spawn_character_button::SpawnCharacterButton, store::AppStore,
    },
};
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CharacterPartSelectionMenu)]
pub fn character_part_selection_menu() -> Html {
    let (app_state, _) = use_store::<AppStore>();

    html!(
        <ul class="p-2 w-fit border border-slate-400 pointer-events-auto" >
        <li>
            <CharacterSelectInput />
        </li>
        <li>
            <SpawnCharacterButton />
        </li>
        {app_state.parts_available.heads.iter()
            .map(|item| html!(
                <li>
                    <SelectCharacterPartButton
                        name={item.clone()}
                        category={CharacterPartCategories::Head}
                    />
                </li>
            ))
            .collect::<Html>()}
        {app_state.parts_available.torsos.iter()
            .map(|item| html!(
                <li>
                    <SelectCharacterPartButton
                        name={item.clone()}
                        category={CharacterPartCategories::Torso}
                    />
                </li>
            ))
            .collect::<Html>()}
        {app_state.parts_available.legs.iter()
            .map(|item| html!(
                <li>
                    <SelectCharacterPartButton
                        name={item.clone()}
                        category={CharacterPartCategories::Leg}
                    />
                </li>
            ))
            .collect::<Html>()}

        </ul>
    )
}
