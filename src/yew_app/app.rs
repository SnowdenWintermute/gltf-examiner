use std::ops::Deref;

use super::Props;
use crate::comm_channels::{CounterEvent, MessageFromBevy, MessageFromYew};
use crossbeam_channel::Receiver;
use gloo::{console::log, timers::callback::Timeout};
use rand::Rng;
use yew::{platform::spawn_local, prelude::*};

pub fn check_for_messages(
    receiver: Receiver<MessageFromBevy>,
    num_messages_state: UseStateHandle<i32>,
    timer_state: UseStateHandle<Option<Timeout>>,
) {
    for message_from_bevy in receiver.try_iter() {
        match message_from_bevy {
            MessageFromBevy::Text(text) => log!(format!("got messagee from bevy: {text}")),
        };
        num_messages_state.set(*num_messages_state + 1);
    }
    let cloned_receiver = receiver.clone();
    let cloned_timer_state = timer_state.clone();
    let cloned_num_messages_state = num_messages_state.clone();
    timer_state.set(Some(Timeout::new(100, move || {
        check_for_messages(
            cloned_receiver,
            cloned_num_messages_state,
            cloned_timer_state,
        )
    })))
}

#[function_component(App)]
pub fn app(props: &Props) -> Html {
    let counter_state = use_state(|| 0);
    let transmitter = props.transmitter.clone();
    let name = props.shared.lock().unwrap().name.clone();
    let num_messages_received_state = use_state(|| 0);
    let new_bevy_messages_state = use_state(|| Vec::new());
    let queued_bevy_messages_state: UseStateHandle<Vec<MessageFromBevy>> = use_state(|| Vec::new());

    let mut receiver = props.bevy_transmitter.subscribe();
    use_effect_with((), {
        let num_messages_received_state = num_messages_received_state.clone();
        let new_bevy_messages_state = new_bevy_messages_state.clone();
        move |()| {
            let num_messages_received_state = num_messages_received_state.clone();
            let new_bevy_messages_state = new_bevy_messages_state.clone();
            spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    // log!("new message from bevy");
                    // let mut rng = rand::thread_rng();
                    // let random_num = rng.gen_range(0..100);
                    //
                    new_bevy_messages_state.set(Vec::from([message]));
                    let old_state = *num_messages_received_state;
                    num_messages_received_state.set(old_state + 1);
                }
            });
        }
    });

    let cloned_queued_bevy_messages_state = queued_bevy_messages_state.clone();
    let cloned_new_bevy_messages_state = new_bevy_messages_state.clone();
    use_effect_with(new_bevy_messages_state, move |_| {
        let mut messages_to_enqueue = cloned_new_bevy_messages_state.deref().clone();
        let mut current_messages = cloned_queued_bevy_messages_state.deref().clone();
        current_messages.append(&mut messages_to_enqueue);
        if current_messages.len() > 10 {
            let (removed, current_messages) = current_messages.split_at(10);
            cloned_queued_bevy_messages_state.set(current_messages.to_vec());
        } else {
            cloned_queued_bevy_messages_state.set(current_messages);
        }
        cloned_new_bevy_messages_state.set(Vec::new());
    });

    let cloned_counter_state = counter_state.clone();
    let handle_click = Callback::from(move |_| {
        log!("clicked");
        let value = *cloned_counter_state + 1;
        cloned_counter_state.set(value);
        transmitter
            .send(MessageFromYew::Counter(CounterEvent { value }))
            .expect("could not send event");
    });

    let mut messages_to_display = Vec::new();

    for message in queued_bevy_messages_state.deref() {
        match message {
            MessageFromBevy::Text(text) => {
                messages_to_display.push(html!(<span>{format!("{text}, ")}</span>))
            }
        };
    }

    html! {
        <main>
            <div class="text-white">
                <button onclick={handle_click} class="h-10 w-60 border border-white " >{ "+1" }</button>
                <p>{ *counter_state }</p>
                // {
                    // messages_to_display
                // }
            </div>
        </main>
    }
}
