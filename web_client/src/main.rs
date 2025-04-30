mod access_handler;
mod base_components;
mod base_hooks;
mod bindings;
mod config;
mod navigation;
mod not_found;
mod refresh_request;
mod request;
mod utils;

use crate::base_components::sidebar::Sidebar;
use crate::access_handler::get_access;
use crate::base_components::page::Page;
use crate::base_components::sidebar_nav_item_content::SidebarNavItemContent;
use crate::config::{set_config, Config};
use crate::navigation::NavigationItem;
use crate::navigation::NavigationItemGroup;
use crate::request::{Request, RequestConfig};
use web_sys::js_sys::Math::random;
use yew::platform::spawn_local;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let counter = counter.clone();
            Request::post("/increment", Some(*counter), move |res| {
                counter.set(res);
            })
        }
    };

    html! {
        <>
        <Sidebar>
            <NavigationItemGroup class="sidebar-nav-group" url="/">
                    <NavigationItem url="/home"><SidebarNavItemContent src="/static/images/home.svg">{"Home"}</SidebarNavItemContent></NavigationItem>
                <div class="spacer"/>
                <NavigationItem url="/settings"><SidebarNavItemContent src="/static/images/settings.svg">{"Settings"}</SidebarNavItemContent></NavigationItem>
            </NavigationItemGroup>
        </Sidebar>
        <div>
            <div class="top-bar">
                <div class="top-bar-navigation">
                    <NavigationItemGroup url="/home">
                        <NavigationItem url="/">{"Home"}</NavigationItem>
                        <NavigationItem url="/dashboard">{"Dashboard"}</NavigationItem>
                        <NavigationItem url="/user">{"User"}</NavigationItem>
                    </NavigationItemGroup>
                    <NavigationItemGroup url="/settings">
                        <NavigationItem url="/">{"Settings"}</NavigationItem>
                        <NavigationItem url="/profile">{"Profile"}</NavigationItem>
                        <NavigationItem url="/language">{"Language"}</NavigationItem>
                    </NavigationItemGroup>
                </div>
            </div>
            <main class="app-main">
                <h1>{"Hello Worlds"}</h1>
                <div>
                    <button {onclick}>{ "+1" }</button>
                    <p>{ *counter }</p>
                    <p>{format!("Random stuff: {}", random())}</p>
                </div>
                <h1>{"This is the page itself:"}</h1>
                <div>
                    <Page/>
                </div>
            </main>
        </div>
        </>
    }
}

fn main() {
    let (routes, default_routes) = routes!(
        "/" => {
            <>
            <h1>{"MAIN PAGE"}</h1>
            </>
        },
        "/home" => [
            "/" => {"Welcome Home"},
            "/user" => {"This is your profile"},
            "/dashboard" => {"The most essential info out there... would be here"},
        ],
        "/settings" => [
            "/" => {"Welcome to settings"},
            "/profile" => {"Delete yourself??"},
            "/language" => {"En/Ru"},
        ],
    );
    let config = Config {
        base_url: "/app",
        routes,
        default_routes,
    };
    RequestConfig::init(RequestConfig::with_default_messages());
    set_config(config);
    spawn_local(async {
        get_access().await.unwrap();
    });
    yew::Renderer::<App>::new().render();
}
