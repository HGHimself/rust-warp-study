pub fn document(title: String, content: String) -> String {
    let head = head(title);
    let header = header();
    format!(
        "<!DOCTYPE html>
<html lang='en'>
{head}
<body>
{header}
{content}
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
        <h1 class='title'>digitheque.io</h1>
    </header>",
    )
}
