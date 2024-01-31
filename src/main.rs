mod components;
mod pages;

use pages::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
