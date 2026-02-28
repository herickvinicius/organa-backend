use axum::http::{
    HeaderMap,
    header,
};
use time::Duration;

// read refresh_cookie
pub fn read_refresh_cookie(
    headers: &HeaderMap,
) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;

    cookie_header
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            cookie
                .strip_prefix("refresh_token=")
                .map(|v| v.to_string())
        })
}

//set refresh cookie
pub fn set_refresh_cookie(
    headers: &mut HeaderMap,
    refresh_token: &str,
    ttl: Duration,
) {
    let cookie = format!(
        "refresh_token={}; HttpOnly; Path=/auth/refresh; Max-Age={}",
        refresh_token,
        ttl.whole_seconds()
    );

    headers.insert(
        header::SET_COOKIE,
        cookie.parse().unwrap(),
    );
}