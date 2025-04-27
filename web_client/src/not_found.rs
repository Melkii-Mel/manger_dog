use js_sys::Math::random;
use yew::{html, Html};

pub fn not_found() -> Html {
    html!(
        <h1>{format!("We did not find this resource, but maybe yew will... Anyways, here's some random number just in case: {}", random())}</h1>
    )
}