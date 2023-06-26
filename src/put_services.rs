use actix_multipart::{Field, Multipart};
use actix_web::web::{Bytes, BytesMut};
use actix_web::{put, Error, HttpResponse};
use futures::StreamExt;
use regex::Regex;
use tokio::fs::write;
use tokio_util::codec;

use crate::fs::read_json;
use crate::upload::upload;

fn get_filename_from_field(field: &Field) -> &str {
    match field.content_disposition().get_filename() {
        None => "",
        Some(f) => f,
    }
}

fn is_correct_filename(filename: &str) -> bool {
    let regex = Regex::new(r#"(?i)\.(mjs|js|wasm)$"#).unwrap();
    filename != "" && regex.is_match(filename)
}

async fn get_file_content(field: &mut Field) -> BytesMut {
    let mut body: BytesMut = BytesMut::new();
    while let Some(chunk) = field.next().await {
        let chunk = chunk.expect("failed to get chunk");
        body.extend_from_slice(&chunk);
    }
    body
}

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}")]
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(field) = payload.next().await {
        let mut f = field.expect("failed to get fields");

        let file_content = get_file_content(&mut f).await;
        let filename: &str = get_filename_from_field(&f);

        if is_correct_filename(filename) {
            //write(filename, &file_content).await?;
            upload(filename, file_content).await;
        }
    }

    Ok(HttpResponse::Ok().body(read_json(
        "./src/defaultResponses/put_accounts_scripts.json",
    )))
}
