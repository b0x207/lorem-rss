use axum::{
    Router,
    http::header::{CONTENT_TYPE, HeaderMap, HeaderValue},
    routing::get,
};
use fake::Fake;

async fn root() -> (HeaderMap, String) {
    let now = chrono::Utc::now();
    let lorem_title = {
        let faker = fake::faker::lorem::en::Words(5..10);
        faker.fake::<Vec<String>>().join(" ")
    };
    let lorem_body = {
        let faker = fake::faker::lorem::en::Paragraphs(5..10);
        faker.fake::<Vec<String>>().join("\n")
    };
    let channel = rss::ChannelBuilder::default()
        .title("Lorem Ipsum RSS")
        .items(vec![
            rss::ItemBuilder::default()
                .title(Some(lorem_title))
                .content(Some(lorem_body))
                .pub_date(Some(now.to_rfc2822()))
                .build(),
        ])
        .build();

    let string = channel.to_string();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));

    (headers, string)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let port = std::env::args().nth(1).unwrap_or("3000".to_string());

    println!("Listening on port {port}");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
