use tower_cookies::cookie::{Cookie, SameSite, time::Duration};

pub fn set_refresh_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("refresh_token", token))
        .secure(false) // true in production (HTTPS)
        .http_only(true)
        .same_site(SameSite::Lax)
        // .domain("http://0.0.0.0:7400")
        .path("/")
        .max_age(Duration::days(1))
        .build()
}

pub fn clear_refresh_cookies() -> Cookie<'static> {
    Cookie::build(("refresh_token", ""))
        .secure(false) // true in production (HTTPS)
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::days(30))
        .build()
}
// .domain("www.rust-lang.org")
