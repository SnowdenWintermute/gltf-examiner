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

    //let cloned_transmitter
    //use_effect_with((), move |_| {
    //    //
    //});

    let state = use_state(|| String::new());
    let mut receiver = props.bevy_transmitter.subscribe();
    use_effect_with((), {
        let state = state.clone();
        let num_messages_received_state = num_messages_received_state.clone();
        move |()| {
            let num_messages_received_state = num_messages_received_state.clone();
            let state = state.clone();
            spawn_local(async move {
                while let Ok(message) = receiver.recv().await {
                    log!("new message from bevy");
                    state.set("new_message".to_string());
                    let mut rng = rand::thread_rng();
                    let random_num = rng.gen_range(0..100);
                    num_messages_received_state.set(random_num);
                }
            });
        }
    });

    let cloned_message_listener_timer_state = message_listener_timer_state.clone();
    let cloned_num_messages_received_state = num_messages_received_state.clone();
    let cloned_receiver = props.receiver.clone();
    use_effect_with((), move |_| {
        let cloned_cloned_message_listener_timer_state =
            cloned_message_listener_timer_state.clone();
        cloned_message_listener_timer_state.set(Some(Timeout::new(1, move || {
            check_for_messages(
                cloned_receiver,
                cloned_num_messages_received_state,
                cloned_cloned_message_listener_timer_state,
            )
        })));
    });

    // // use_effect_with((), f)

    let cloned_counter_state = counter_state.clone();
    let handle_click = Callback::from(move |_| {
        let value = *cloned_counter_state + 1;
        cloned_counter_state.set(value);
        transmitter
            .send(MessageFromYew::Counter(CounterEvent { value }))
            .expect("could not send event");
    });

    html! {
        <main>
            <div class="text-white">
                <button onclick={handle_click} >{ "+1" }</button>
                <p>{ *counter_state }</p>
                {"ay"}
        {format!("{:?}",*num_messages_received_state)}
            </div>
        </main>
    }
}
