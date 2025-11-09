use actix_web::{http::header::ContentType, HttpResponse};

pub async fn submit_newsletter_form() -> HttpResponse {
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
        <form action="/admin/newsletters" method="post">
            <label>Title
                <input
                    type="text"
                    placeholder="Enter title"
                    name="title"
                >
            </label>
            <label>Content
                <input
                    type="text"
                    placeholder="Enter newsletter content"
                    name="content"
                >
            </label>
            <button type="submit">Submit</button>
        </form>
    </body>
</html>"#
        ))
}
