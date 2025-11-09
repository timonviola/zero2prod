use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn submit_newsletter_form(flash_message: IncomingFlashMessages ) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_message.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Submit a newsletter issue!</title>
    </head>
    <body>
        {msg_html}
        <form action="/admin/newsletters" method="post">
            <label>Title
                <input
                    type="text"
                    placeholder="Enter title"
                    name="title"
                >
            </label>
            <label>Text Content
                <input
                    type="text"
                    placeholder="Enter newsletter content"
                    name="text_content"
                >
            </label>
            <label>HTML Content
                <input
                    type="text"
                    placeholder="Enter newsletter content"
                    name="html_content"
                >
            </label>
            <button type="submit">Submit</button>
        </form>
        <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>"#
        ))
}
