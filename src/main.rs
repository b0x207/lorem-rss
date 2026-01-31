use std::time::Instant;

use axum::{
    Router,
    extract::State,
    http::header::{CONTENT_TYPE, HeaderMap, HeaderValue},
    routing::get,
};
use fake::Fake;

const TIME_RATE: i64 = 60; // Once a minute

#[derive(Clone)]
struct AppState {
    start: chrono::DateTime<chrono::Utc>,
}

async fn root(State(state): State<AppState>) -> (HeaderMap, String) {
    let count = {
        let time_delta = chrono::Utc::now() - state.start;
        time_delta.num_seconds() / TIME_RATE
    };
    let now = chrono::Utc::now();
    let item_builder = |i| {
        let lorem_title = {
            let faker = fake::faker::lorem::en::Words(5..10);
            faker.fake::<Vec<String>>().join(" ")
        };
        let lorem_body = {
            let faker = fake::faker::lorem::en::Paragraphs(5..10);
            faker.fake::<Vec<String>>().join("\n")
        };
        let item_time = now - chrono::Duration::seconds(TIME_RATE * i as i64);
        rss::ItemBuilder::default()
            .title(Some(lorem_title))
            .content(Some(lorem_body))
            .pub_date(Some(item_time.to_rfc2822()))
            .build()
    };
    let channel = rss::ChannelBuilder::default()
        .title("Lorem Ipsum RSS")
        .items((0..count).map(item_builder).collect::<Vec<rss::Item>>())
        .build();

    let string = channel.to_string();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));

    (headers, string)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        start: chrono::Utc::now(),
    };
    let app = Router::new().route("/", get(root)).with_state(state);
    let port = std::env::args().nth(1).unwrap_or("3000".to_string());

    println!("Listening on port {port}");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
