use super::store::AppStore;
use super::Props;
use crate::comm_channels::MessageFromBevy;
use crate::yew_app::character_part_selection_menu::CharacterPartSelectionMenu;
// use gloo::console::info;
// use gloo::console::log;
use std::ops::Deref;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let (_, dispatch) = use_store::<AppStore>();
    let Props {
        bevy_transmitter, ..
    } = props;

    let cloned_transmitter = props.transmitter.clone();
    let cloned_dispatch = dispatch.clone();
    use_effect_with((), move |_| {
        cloned_dispatch.reduce_mut(|store| store.transmitter_option = Some(cloned_transmitter))
    });

    // let name = shared.lock().unwrap().name.clone();
    let most_recent_message_from_bevy_state = use_state(|| Vec::new());
    let queued_bevy_messages_state: UseStateHandle<Vec<MessageFromBevy>> = use_state(|| Vec::new());

    // GET THE MOST RECENT MESSAGE
    let mut receiver = bevy_transmitter.subscribe();
    use_effect_with((), {
        let most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
        move |()| {
            let most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
            spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    let mut messages = Vec::from([message]);
                    while let Ok(subsequent_message) = receiver.try_recv() {
                        messages.push(subsequent_message)
                    }
                    // log!(format!("got messages from bevy: {:#?}", messages));
                    most_recent_message_from_bevy_state.set(messages);
                }
            });
        }
    });

    // READ THE MOST RECENT MESSAGE AND ADD TO QUEUE
    let cloned_queued_bevy_messages_state = queued_bevy_messages_state.clone();
    let cloned_most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
    use_effect_with(most_recent_message_from_bevy_state, move |_| {
        let mut message_to_enqueue = cloned_most_recent_message_from_bevy_state.deref().clone();

        // log!(format!("enqueuing message : {:#?}", message_to_enqueue));
        let mut current_messages = cloned_queued_bevy_messages_state.deref().clone();
        current_messages.append(&mut message_to_enqueue);
        cloned_queued_bevy_messages_state.set(current_messages);
        cloned_most_recent_message_from_bevy_state.set(Vec::new());
    });

    // DEQUEUE AND HANDLE MESSAGES
    let cloned_queued_bevy_messages_state = queued_bevy_messages_state.clone();
    let cloned_dispatch = dispatch.clone();
    use_effect_with(
        cloned_queued_bevy_messages_state.clone(),
        move |cloned_queued_bevy_messages_state| {
            let messages = cloned_queued_bevy_messages_state.deref();
            for message in messages {
                // log!(format!("processing message {:?}", message));
                match message {
                    MessageFromBevy::PartNames(part_names) => cloned_dispatch
                        .reduce_mut(|store| store.parts_available = part_names.clone()),
                    MessageFromBevy::AnimationsAvailable(animation_names) => cloned_dispatch
                        .reduce_mut(|store| store.animation_names = animation_names.clone()),
                    MessageFromBevy::CombatantSpawned(combatant_id) => {
                        cloned_dispatch.reduce_mut(|store| store.character_ids.push(*combatant_id))
                    }
                    MessageFromBevy::AssetsLoaded => {
                        // info!("setting assets loaded");
                        cloned_dispatch.reduce_mut(|store| store.bevy_assets_loaded = true)
                    }
                }
            }
            cloned_queued_bevy_messages_state.set(Vec::new());
        },
    );

    html! {
        <main class="text-zinc-300" >
            {queued_bevy_messages_state.deref().iter().map(|item| html!(<div>{format!("{:#?}", item)}</div>)).collect::<Html>()}
            <CharacterPartSelectionMenu />
        </main>
    }
}
