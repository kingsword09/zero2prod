use actix_web::{cookie::Cookie, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        _ = writeln!(error_html, "<p><i>{}</i></p>", m.content());
    }

    let mut response = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Type" content="text/html;charset=UTF-8">
  <title>Login</title>
</head>
<body>
  {error_html}
  <form action="/login" method="post">
    <label>Username
      <input type="text" placeholder="Enter Username" name="username" />
    </label>

    <label>Password
      <input type="password" placeholder="Enter Password" name="password" />
    </label>

    <button type="submit">Login</button>
  </form>
</body>
</html>"#
        ));

    response
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();
    response
}
