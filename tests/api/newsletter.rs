use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn successfully_submit_newsletter_form() {
    // I. Access submit form
    let app = spawn_app().await;

    app.test_user.login(&app).await;

    let response = app.get_submit_newsletters().await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return the form.",
    );

    let html_page = app.get_submit_newsletters_html().await;
    assert!(html_page.contains(r#"<form action="/admin/newsletters" method="post">"#));
    // II. Submit form
    // III. Assert form is saved after user clicked submit.
    // flash message should show if something went wrong
}

#[tokio::test]
async fn submit_newsletters_returns_form() {
    let app = spawn_app().await;

    app.test_user.login(&app).await;

    let response = app.get_submit_newsletters().await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return the form.",
    );

    let html_page = app.get_submit_newsletters_html().await;
    assert!(html_page.contains(r#"<form action="/admin/newsletters" method="post">"#));
}

#[tokio::test]
async fn admin_dashboard_includes_link_to_newsletters() {
    let app = spawn_app().await;

    app.test_user.login(&app).await;

    let html_page = app.get_admin_dashboard_html().await;

    assert!(html_page.contains(r#"<a href="/admin/newsletters">Send a newsletter</a>"#));
}

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_newsletters_form() {
    let app = spawn_app().await;
    let response = app.get_submit_newsletters().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_submit_newsletter() {
    let app = spawn_app().await;

    let newsletter_request_body = &serde_json::json!({
      "title": "Newsletter title",
      "text_content": "Newsletter body as plain text",
      "html_content": "<p>Newsletter body as HTML</p>",
    });

    let response = app.post_newsletters(newsletter_request_body).await;

    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.test_user.login(&app).await;
    let response = app
        .post_newsletters(&serde_json::json!({
          "title": "Newsletter title",
          "text_content": "Newsletter body as plain text",
          "html_content": "<p>Newsletter body as HTML</p>",
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/newsletters");
    let html_page = app.get_submit_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    // Mock verifies on Drop that we have sent the newsletter email
}

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = spawn_app().await;

    app.test_user.login(&app).await;

    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
      "title": "Newsletter title",
      "text_content": "Newsletter body as plain text",
      "html_content": "<p>Newsletter body as HTML</p>",
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");
    let html_page = app.get_submit_newsletters_html().await;
    // assert on injected FlashMessage content
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    // Mock verifies on Drop that we haven't sent the newsletter email
}

#[tokio::test]
async fn newsletters_return_400_for_invalid_data() {
    let app = spawn_app().await;

    app.test_user.login(&app).await;

    let test_cases = vec![
        (
            serde_json::json!({
                "text_content": "Newsletter body as plain text",
                "html_content": "<p>Newsletter body as HTML</p>",
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Newsletter!"}),
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_newsletters(&invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=le%20guin&email=cica%40gmail.com";
    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    //inspect mock confirmation link
    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
