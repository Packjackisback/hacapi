//The purpose of this module is to get the login (.AuthCookie) cookie to be used on future requests, along with providing the ASP.net session id and the long cookie
use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router
};
use axum::routing::head;
use reqwest::Client;
use reqwest::cookie::Cookie;
use tower_http::trace::TraceLayer;
use reqwest::header;
use scraper::{Html, Selector};
//This should return the ASP.NET_SessionID, request id, and hidden request id, in that order
pub async fn get_session_cookie() -> (String, String, String) {
    let client = Client::new();
    let url = String::from("https://homeaccess.katyisd.org/HomeAccess/Account/LogOn");
    let response = client.get(url).send().await.unwrap();
    let mut cookies = (String::from(""), String::from(""), String::from(""));
    for c in response.cookies() {
        match(c.name()) {
            "__RequestVerificationToken_L0hvbWVBY2Nlc3M1" => {
                cookies.1 = String::from(c.value());
            }
            "ASP.NET_SessionId" => {
                cookies.0 = String::from(c.value());
            }
            _ => {}
        }

    }
    let text = response.text().await.expect("error");
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#".container > form:nth-child(1) > input:nth-child(1)"#).expect("Failed to create selector");
    for element in document.select(&selector) {
        if let Some(value) = element.value().attr("value") {
            println!("Input value: {}", value);
            cookies.2 = String::from(value);
        }
    }
    return cookies;
}

pub async fn get_login_cookie(session_cookies: (String, String, String), username: &str, password: &str) -> String {
    let asp_session_id = session_cookies.0;
    let ver_token = session_cookies.1;
    let ver_hidden_token = session_cookies.2;
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0".parse().unwrap());
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br, zstd".parse().unwrap());
    headers.insert("Referer", "https://homeaccess.katyisd.org/HomeAccess/Account/LogOn?ReturnUrl=%2fhomeaccess".parse().unwrap());
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("Origin", "https://homeaccess.katyisd.org".parse().unwrap());
    headers.insert("DNT", "1".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    let cookie_header = String::from("ASP.NET_SessionId=".to_owned() + asp_session_id.as_str() + "; SPIHACSiteCode=; __RequestVerificationToken_L0hvbWVBY2Nlc3M1=" + ver_token.as_str());
    headers.insert(header::COOKIE, cookie_header.parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
    headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
    headers.insert("Priority", "u=0, i".parse().unwrap());
    let client = reqwest::Client::builder()
        .build()
        .unwrap();

    let body = format!(
        "__RequestVerificationToken={}&SCKTY00328510CustomEnabled=True&SCKTY00436568CustomEnabled=True&Database=10&VerificationOption=UsernamePassword&LogOnDetails.UserName={}&tempUN=&tempPW={}&LogOnDetails.Password={}",
        ver_hidden_token, username, password, password
    );
    // println!("{:?}", headers);
    // println!("{}", body);
    let res = client.post("https://homeaccess.katyisd.org/HomeAccess/Account/LogOn?ReturnUrl=%2fHomeAccess%2f")
        .headers(headers)
        .body(body)
        .send()
        .await;
    for cookie in res.unwrap().cookies() {
        println!("{} - {}", cookie.name(), cookie.value());
        if(cookie.name().eq(".AuthCookie")) {
            println!("{} = {}", cookie.name(), cookie.value());
            return String::from(cookie.value());
        }
    }
    return String::from("No Token Found");
}




fn hash() {

}