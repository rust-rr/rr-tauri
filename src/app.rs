use crate::components::side_bar::SideBar;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div class="h-[100%] flex">
            <SideBar />
            <div class="flex-1 px-5 py-4">
                { "Hello world" }
            </div>
        </div>
    )
}
