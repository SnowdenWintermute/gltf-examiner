use yew::prelude::*;

// #[derive(Properties)]
// pub struct Props {
// pub shared: Shared<SharedState>,
// pub transmitter: YewTransmitter,
// pub receiver: YewReciever,
// }

#[function_component(App)]
pub fn app() -> Html {
    let counter_state = use_state(|| 0);
    // let transmitter = props.transmitter_events.clone();
    // let name = props.shared.lock().unwrap().name.clone();

    let cloned_counter_state = counter_state.clone();
    let handle_click = Callback::from(move |_| {
        let value = *cloned_counter_state + 1;
        cloned_counter_state.set(value);
        // transmitter
        //     .send(crate::events::InputEvent::Counter(CounterEvent { value }))
        //     .expect("could not send event");
    });

    // <p>{ shared_state_name }</p>

    html! {
        <main>
            <div>
                <button onclick={handle_click} >{ "+1" }</button>
                <p>{ *counter_state }</p>
            </div>
        </main>
    }
}

pub fn yew_main() {
    let document = gloo::utils::document();
    let root = document.query_selector("#yew").unwrap().unwrap();
    yew::Renderer::<App>::with_root_and_props(
        root,
        (), // Props {
            //     transmitter_events,
            //     receiver_events,
            //     shared,
            // },
    )
    .render();
}
