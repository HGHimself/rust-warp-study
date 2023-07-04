use std::include_str;

use crate::models;

pub fn document(title: String, content: String) -> String {
    let head = head(title);
    let footer = footer();
    let header = header();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
{head}
<body>
{header}
{content}
{footer}
</body>
</html>"
    )
}

pub fn head(title: String) -> String {
    format!(
        "<head>
<meta name='viewport' content='width=device-width, initial-scale=1' />
<meta charset='utf-8' />
<title>{title}</title>
<link rel='stylesheet' href='/static/style.css' />
<link rel='stylesheet' href='/static/mobile.css' media='screen and (max-width: 600px)' />
<link rel='icon' type='image/svg+xml' href='/static/favicon.svg' sizes='any' />
<script src='https://unpkg.com/htmx.org@1.9.2' integrity='sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h' crossorigin='anonymous'></script>
</head>",
    )
}

pub fn header() -> String {
    String::from(
        "
    <header>
    <h1 class='title'><a href='/' class='normalized'>digitheque.io</a></h1>
    <a href='/user/login'>Login</a>
    </header>",
    )
}

pub fn header_authenticated(user: &models::user::User) -> String {
    format!(
        "
    <header>
    <h1 class='title'><a href='/' class='normalized'>digitheque.io</a></h1>
    <a href='/user'>{}</a>
    <a href='/user/logout'>Logout</a>
    </header>",
        user.username
    )
}

pub fn index_authenticated(user: &models::user::User, message: &str) -> String {
    document_authenticated(
        String::from("Digitheque"),
        user,
        include_str!("index-authenticated.html")
            .to_string()
            .replace("{message}", message),
    )
}

pub fn index(message: &str) -> String {
    document(
        String::from("Digitheque"),
        include_str!("index.html")
            .to_string()
            .replace("{message}", message),
    )
}

pub fn document_authenticated(title: String, user: &models::user::User, content: String) -> String {
    let head = head(title);
    let footer = footer();
    let header = header_authenticated(user);
    format!(
        "<!DOCTYPE html>
<html lang='en'>
{head}
<body>
{header}
{content}
{footer}
</body>
</html>"
    )
}

fn footer() -> String {
    String::from("<footer>
    created by HG King | © 2023 Digitheque.io | <a href='https://github.com/hgm-king/rust-warp-study' target='_blank' >code</a>
</footer>")
}
