use crate::{
    frontend_common::CharacterPartCategories,
    yew_app::{select_character_part_button::SelectCharacterPartButton, store::AppStore},
};
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CharacterPartSelectionMenu)]
pub fn character_part_selection_menu() -> Html {
    let (app_state, _) = use_store::<AppStore>();

    html!(
        <ul>
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
