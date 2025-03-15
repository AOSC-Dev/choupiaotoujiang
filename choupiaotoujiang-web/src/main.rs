use std::io::{self, BufReader, Cursor};

use anyhow::Context;
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use sha2::{Digest, Sha512};
use tower_http::cors::{Any, CorsLayer};

pub struct AnyhowError(anyhow::Error);

impl IntoResponse for AnyhowError {
    fn into_response(self) -> Response {
        println!("Returing internal server error for {}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.0)).into_response()
    }
}

impl<E> From<E> for AnyhowError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let uri = std::env::var("choujiang_uri")?;

    let router = Router::new()
        .route("/", get(home))
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::disable())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods([Method::POST]),
        );

    let listener = tokio::net::TcpListener::bind(&uri).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn home() -> Html<&'static str> {
    Html::from(include_str!("../choujiang.html"))
}

async fn upload(mut form: Multipart) -> Result<Json<u32>, AnyhowError> {
    let mut v = vec![];
    let mut random_num_start = None;
    let mut total = None;
    while let Some(field) = form.next_field().await? {
        match field.name() {
            Some("file") => {
                v.extend(field.bytes().await?);
            }
            Some("num_start") => {
                let num = field.text().await?.parse::<u32>()?;
                random_num_start = Some(num);
            }
            Some("total") => {
                let num = field.text().await?.parse::<u32>()?;
                total = Some(num);
            }
            _ => continue,
        }
    }

    let random_num_start = random_num_start.context("Request has no 'num_start' field.")?;
    let total = total.context("Request has no 'total' field.")?;

    let num = tokio::task::spawn_blocking(move || get_num(v, random_num_start, total)).await??;

    Ok(Json::from(num))
}

fn get_num(f: Vec<u8>, start: u32, peoples: u32) -> anyhow::Result<u32> {
    let f = Cursor::new(f);
    let mut reader = BufReader::new(f);
    let mut sha512 = Sha512::new();
    io::copy(&mut reader, &mut sha512)?;
    let v = sha512.finalize();
    let mut rng: Pcg64 = Seeder::from(&v).into_rng();

    Ok(rng.random_range(start..=peoples))
}
