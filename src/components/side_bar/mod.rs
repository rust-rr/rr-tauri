use search_bar::SearchBar;
use yew::{function_component, html, Html};

mod classification;
mod lists;
mod search_bar;
mod tags;

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html!(
        <div class="w-[20%] border-gray-600 border-r-[1px] px-5 py-4">
            <SearchBar />
            { "I'm side bar" }
        </div>
    )
}
