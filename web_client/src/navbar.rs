use yew::{html, Html};

pub fn navbar(items: Vec<String>) -> Html {
    html!(
        <nav>
            { for items.iter().map(|item| html!(<button>{item}</button>)) }
        </nav>
    )
}