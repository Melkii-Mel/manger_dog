mod access_handler;
mod components;
mod hooks;
mod bindings;
mod config;
mod i18n;
mod navigation;
mod not_found;
mod refresh_request;
mod request;
mod utils;

use crate::request::Request;
use crate::access_handler::get_access;
use crate::components::page::Page;
use crate::components::sidebar::Sidebar;
use crate::components::sidebar_nav_item_content::SidebarNavItemContent;
use crate::hooks::use_translation::use_t;
use crate::config::{get_config, set_config, Config};
use crate::i18n::translation_config;
use crate::navigation::NavigationItem;
use crate::navigation::NavigationItemGroup;
use crate::request::{RRequest, RequestConfig};
use i18nrs::yew::I18nProvider;
use web_sys::js_sys::Math::random;
use yew::platform::spawn_local;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component]
fn AppWithConfig() -> Html {
    let config = get_config();

    let translation_config =
        translation_config(&config.translations_string, &config.default_language);
    html!(
        <I18nProvider ..translation_config>
            <App/>
        </I18nProvider>
    )
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let counter = counter.clone();
            RRequest::post("/increment".to_string(), Some(*counter), move |res| {
                counter.set(res);
            })
        }
    };

    let t = use_t();

    html! {
        <>
        <Sidebar>
            <NavigationItemGroup class="sidebar-nav-group" url="/">
                    <NavigationItem url="/home"><SidebarNavItemContent src="/static/images/home.svg">{t("home")}</SidebarNavItemContent></NavigationItem>
                <div class="spacer"/>
                <NavigationItem url="/settings"><SidebarNavItemContent src="/static/images/settings.svg">{t("settings")}</SidebarNavItemContent></NavigationItem>
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
    console_error_panic_hook::set_once();
    spawn_local(async {
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
        let translations_string = Request::get_body_unchecked("/static/translations.json")
            .await
            .unwrap();
        let config = Config {
            base_url: "/app",
            routes,
            default_routes,
            translations_string,
            default_language: "ru".to_string(),
            authorized_locations: &["/api/"],
        };
        set_config(config);
        get_access().await.unwrap();
        RequestConfig::init(RequestConfig::with_default_messages());
        yew::Renderer::<AppWithConfig>::new().render();
    });
}
