use crate::comm_channels::MessageFromYew;
use crate::frontend_common::AttackCommand;
use crate::yew_app::store::AppStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(ExecuteAttackSequenceButton)]
pub fn execute_attack_sequence_button() -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();

    let cloned_app_state = app_state.clone();
    let handle_click = Callback::from(move |_| {
        if let Some(transmitter) = &cloned_app_state.transmitter_option {
            dispatch.reduce_mut(|_| {
                transmitter
                    .send(MessageFromYew::ExecuteAttackSequence(AttackCommand {
                        combatant_id: cloned_app_state.selected_character_id,
                        target_id: cloned_app_state.selected_target_id,
                    }))
                    .expect("could not send event");
            });
        }
    });

    html!(
    <button onclick={handle_click} class={format!("h-10 min-w-30 border border-slate-400 bg-slate-700 pr-2 pl-2")}>
        {format!("Attack")}
    </button>
    )
}
