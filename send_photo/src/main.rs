use std::io::{self, BufReader, Cursor};

use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
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
    let peoples = std::env::var("choujiang_peoples")?.parse::<u32>()?;
    let start = std::env::var("choujiang_peoples_start")?.parse::<u32>()?;

    let router = Router::new()
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::disable())
        .with_state((start, peoples))
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

async fn upload(state: State<(u32, u32)>, mut form: Multipart) -> Result<Json<u32>, AnyhowError> {
    let mut v = vec![];
    while let Some(field) = form.next_field().await? {
        match field.name() {
            Some("file") => {
                v.extend(field.bytes().await?);
            }
            _ => continue,
        }
    }

    let num = tokio::task::spawn_blocking(move || get_num(v, state.0 .0, state.0 .1)).await?;

    Ok(Json::from(num))
}

fn get_num(f: Vec<u8>, start: u32, peoples: u32) -> u32 {
    let f = Cursor::new(f);
    let mut reader = BufReader::new(f);
    let mut sha512 = Sha512::new();
    io::copy(&mut reader, &mut sha512).unwrap();
    let v = sha512.finalize();
    let mut rng: Pcg64 = Seeder::from(&v).make_rng();

    rng.gen_range(start - 1..=peoples)
}
