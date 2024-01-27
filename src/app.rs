use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if name.is_empty() {
                    return;
                }

                let args = to_value(&GreetArgs { name: &*name }).unwrap();
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                let new_msg = invoke("greet", args).await.as_string().unwrap();
                greet_msg.set(new_msg);
            });

            || {}
        })
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="flex flex-col justify-center text-center max-w-[100%] pt-[10vh]">
            <div class="flex justify-center">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="h-24 p-6 transform duration-700 hover:scale-125" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="h-24 p-6 transform duration-700 hover:scale-125" alt="Yew logo"/>
                </a>
            </div>

            <p class="mb-4">{"Click on the Tauri and Yew logos to learn more."}</p>

            <p class="mb-4">
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <form class="flex justify-center" onsubmit={greet}>
                <input id="greet-input" class="mr-2" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p class="mt-4"><b>{ &*greet_msg }</b></p>
        </main>
    }
}
