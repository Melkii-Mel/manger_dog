use actix_web::cookie::Cookie;
use actix_web::HttpResponseBuilder;
use time::Duration;

pub trait CookieBuilder {
    fn build_cookie(
        &mut self,
        key: &str,
        value: &str,
        secure: bool,
        path: Option<&str>,
        expiration: Duration,
    ) -> &mut Self;
    fn delete_cookie(&mut self, key: &str) -> &mut Self;
    fn delete_cookies(&mut self, keys: Vec<&str>) -> &mut Self;
}
impl CookieBuilder for HttpResponseBuilder {
    fn build_cookie(
        &mut self,
        key: &str,
        value: &str,
        secure: bool,
        path: Option<&str>,
        expiration: Duration,
    ) -> &mut Self {
        let mut cookie_builder = Cookie::build(key, value)
            .http_only(secure)
            .secure(secure)
            .max_age(expiration);
        if let Some(path) = path {
            cookie_builder = cookie_builder.path(path);
        }
        let cookie = cookie_builder.finish();
        self.cookie(cookie)
    }
    fn delete_cookie(&mut self, key: &str) -> &mut Self {
        self.build_cookie(key, "", false, None, Duration::seconds(0))
    }

    fn delete_cookies(&mut self, keys: Vec<&str>) -> &mut Self {
        keys.iter().for_each(|k| {
            self.delete_cookie(k);
        });
        self
    }
}
