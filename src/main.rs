mod moderator;
use moderator::ContentModerator;
use std::sync::Arc;

use axum::{Router, body::Bytes, routing::post};
use image::{DynamicImage, GenericImageView};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Detection {
    class: String,
    score: f32,
    #[serde(rename = "box")]
    detection_box: [u32; 4],
}

#[derive(Deserialize, Debug)]
struct NudeNetResponse {
    prediction: Vec<Vec<Detection>>,
    success: bool,
}

#[tokio::main]
async fn main() {
    let moderator = Arc::new(ContentModerator::new());

    let app = Router::new().route(
        "/censor",
        post(move |body| handle_proxy(body, moderator.clone())),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server Rust in http://localhost:3000/censor");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_proxy(body: Bytes, moderator: Arc<ContentModerator>) -> Vec<u8> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let form = reqwest::multipart::Form::new().part(
        "f1",
        reqwest::multipart::Part::bytes(body.to_vec()).file_name("i.jpg"),
    );

    let resp = match client
        .post("http://localhost:8080/infer")
        .multipart(form)
        .send()
        .await
    {
        Ok(r) => r.json::<NudeNetResponse>().await.unwrap(),
        Err(e) => {
            eprintln!("Error, can't connect with the model, {}", e);
            return body.to_vec();
        }
    };

    let mut img = image::load_from_memory(&body).unwrap();

    if resp.success {
        if let Some(detections) = resp.prediction.get(0) {
            for det in detections {
                if det.score > 0.3 {
                    img = moderator.process_detection(img, &det.class, det.detection_box);
                }
            }
        }
    }

    let mut buffer = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Jpeg)
        .expect("Error creating buffer");
    buffer.into_inner()
}
