use super::{store::AppStore, Props};
use crate::comm_channels::{MessageFromBevy, MessageFromYew, TextFromYewEvent};
use gloo::console::log;
use std::ops::Deref;
use yew::{platform::spawn_local, prelude::*};
use yewdux::use_store;

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();
    let Props {
        transmitter: yew_transmitter,
        bevy_transmitter,
        shared,
    } = props;

    let counter_state = use_state(|| 0);
    let transmitter = yew_transmitter.clone();
    // let name = shared.lock().unwrap().name.clone();
    let most_recent_message_from_bevy_state = use_state(|| Vec::new());
    let queued_bevy_messages_state: UseStateHandle<Vec<MessageFromBevy>> = use_state(|| Vec::new());

    // GET THE MOST RECENT MESSAGE
    let mut receiver = bevy_transmitter.subscribe();
    use_effect_with((), {
        let most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
        let dispatch = dispatch.clone();
        move |()| {
            let most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
            let dispatch = dispatch.clone();
            spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    log!(format!("got message from bevy: {:#?}", message));
                    most_recent_message_from_bevy_state.set(Vec::from([message.clone()]));
                    match message {
                        MessageFromBevy::Text(_) => todo!(),
                        MessageFromBevy::PartNames(part_names) => {
                            dispatch.reduce_mut(|store| store.parts_available = part_names)
                        }
                    }
                }
            });
        }
    });

    // READ THE MOST RECENT MESSAGE AND ADD TO QUEUE
    let cloned_queued_bevy_messages_state = queued_bevy_messages_state.clone();
    let cloned_most_recent_message_from_bevy_state = most_recent_message_from_bevy_state.clone();
    use_effect_with(most_recent_message_from_bevy_state, move |_| {
        let mut message_to_enqueue = cloned_most_recent_message_from_bevy_state.deref().clone();
        let mut current_messages = cloned_queued_bevy_messages_state.deref().clone();
        current_messages.append(&mut message_to_enqueue);
        cloned_queued_bevy_messages_state.set(current_messages);
        cloned_most_recent_message_from_bevy_state.set(Vec::new());
    });

    // HOW TO USE TRANSMITER
    let cloned_counter_state = counter_state.clone();
    let handle_click = Callback::from(move |_| {
        log!("clicked");
        let value = *cloned_counter_state + 1;
        cloned_counter_state.set(value);
        transmitter
            .send(MessageFromYew::Text(TextFromYewEvent {
                text: format!("{}", value),
            }))
            .expect("could not send event");
    });

    html! {
        <main>
            <div class="text-white">
                <button onclick={handle_click} class="h-10 w-60 border border-white " >{ "+1" }</button>
            </div>
            {queued_bevy_messages_state.deref().iter().map(|item| html!(<div>{format!("{:#?}", item)}</div>)).collect::<Html>()}
        {app_state.parts_available.heads.iter().map(|item| html!(<div>{format!("{:#?}", item)}</div>)).collect::<Html>()}
        </main>
    }
}
