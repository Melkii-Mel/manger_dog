use crate::navigation::DefaultRoutes;
use crate::navigation::Routes;
use std::cell::OnceCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Config {
    pub base_url: &'static str,
    pub routes: Routes,
    pub default_routes: DefaultRoutes,
}

thread_local! {
    static CONFIG: OnceCell<Rc<Config>> = OnceCell::new();
}

pub fn set_config(config: Config) {
    CONFIG.with(|cfg| cfg.set(Rc::new(config)).expect("Config already set"));
}

pub fn get_config() -> Rc<Config> {
    CONFIG.with(|cfg| cfg.get().expect("Config is not set").clone())
}
