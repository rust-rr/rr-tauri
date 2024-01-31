use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let search_input_ref = use_node_ref();
    let search_value = use_state(|| String::new());

    {
        let search_value = search_value.clone();
        use_effect_with(search_input_ref.clone(), {
            let search_input_ref = search_input_ref.clone();
            move |_| {
                let mut keydown_listener = None;
                if let Some(ele) = search_input_ref.cast::<HtmlInputElement>() {
                    let onkeydown = Callback::from(move |e: KeyboardEvent| {
                        if e.key() == "Enter" {
                            search_value
                                .set(search_input_ref.cast::<HtmlInputElement>().unwrap().value());
                        }
                    });

                    let listener = EventListener::new(&ele, "keydown", move |e| {
                        let e = e.dyn_ref::<KeyboardEvent>().unwrap_throw();
                        onkeydown.emit(e.clone());
                    });

                    keydown_listener = Some(listener);
                }

                move || drop(keydown_listener)
            }
        });
    }

    html!(
        <div class="mb-2">
            <input ref={search_input_ref} class="w-[100%]" placeholder="Search" />
            <div>{ &*search_value }</div>
        </div>
    )
}
