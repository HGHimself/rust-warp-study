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
<div>
{header}
<main id='content' class='content'>
{content}
</main>
{footer}
</div>
<script>sal();</script>
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
<link rel='apple-touch-icon' sizes='180x180' href='/apple-touch-icon.png'>
<link rel='icon' type='image/png' sizes='32x32' href='/favicon-32x32.png'>
<link rel='icon' type='image/png' sizes='16x16' href='/favicon-16x16.png'>
<link rel='manifest' href='/site.webmanifest'>
<link rel='mask-icon' href='/safari-pinned-tab.svg' color='#5bbad5'>
<meta name='msapplication-TileColor' content='#da532c'>
<meta name='theme-color' content='#ffffff'>
<meta property='og:title' content='Digitheque'>
<meta property='og:site_name' content='Digitheque'>
<meta property='og:url' content='https://digitheque.io'>
<meta property='og:description' content='A social network allowing for people to explore the internet through a user-provided graph of websites.'>
<meta property='og:type' content='website'>
<meta property='og:image' content='https://digitheque.io/seo/rainbow-logo1.png'>
<link rel='stylesheet' href='/style.css' />
<link rel='stylesheet' href='/mobile.css' media='screen and (max-width: 600px)' />
<script src='https://unpkg.com/htmx.org@1.9.2' integrity='sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h' crossorigin='anonymous'></script>
<script src='https://cdn.jsdelivr.net/npm/d3@7'></script>
<script src='/sal.js'></script>
<link rel='stylesheet' href='/sal.css' />
<script src='/background.js'></script>
</head>",
    )
}

pub fn header() -> String {
    String::from(
        "
    <header>
        <div>
            <h1 class='title'><a href='/' class='normalized'>digitheque.io</a></h1>
            <ul class='actions'>
                <li><a href='/link/random'>Random link</a></li>
                <li><a href='/user/login' class='button-link'>Login</a></li>
            </ul>
        </div>
    </header>",
    )
}

pub fn header_authenticated(user: &models::user::User) -> String {
    format!(
        "
    <header>
        <div>
            <h1 class='title'><a href='/' class='normalized'>digitheque.io</a></h1>
            <ul class='actions'>
                <li><a href='/link/random'>Random link</a></li>
                <li><a href='/user'>{}</a></li>
                <li><a href='/user/logout'>Logout</a></li>
            </ul>
        </div>
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
            .replace("{message}", message)
            .replace("{background}", &models::background::index()),
    )
}

pub fn index(message: &str) -> String {
    document(
        String::from("Digitheque"),
        include_str!("index.html")
            .to_string()
            .replace("{message}", message)
            .replace("{background}", &models::background::index()),
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
<div>
{header}
<main id='content' class='content'>
{content}
</main>
{footer}
</div>
<script>sal();</script>
</body>
</html>"
    )
}

fn footer() -> String {
    String::from("<footer>
    <span>© 2023 Digitheque.io | created by HG King | <a href='https://github.com/hgm-king/rust-warp-study' target='_blank' >code</a></span>
</footer>")
}
