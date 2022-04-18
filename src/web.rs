use actix_web::{get, App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use bili::live::get_play_url_info;

use crate::error::ApiError;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36";

#[get("/{room_id}")]
pub async fn relay(room_id: web::Path<u64>, client: web::Data<reqwest::Client>) -> Result<HttpResponse, ApiError> {
    let room_id = room_id.into_inner();
    let infos = get_play_url_info(room_id).await?;
    if let Some(play_url) = infos.durl.first() {
        let resp = client.get(&play_url.url).send().await?;
        let headers = resp.headers();
        let mut forged = HttpResponse::Ok();
        for header in headers {
            forged.insert_header(header);
        }
        Ok(forged.streaming(resp.bytes_stream()))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn spawn_server() -> anyhow::Result<()> {
    HttpServer::new(move || {
        let client = reqwest::ClientBuilder::default()
            .user_agent(reqwest::header::HeaderValue::from_static(USER_AGENT))
            .build()
            .unwrap();

        App::new()
            .app_data(web::Data::new(client))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                    .into()
            }))
            .wrap(Logger::default())
            .service(relay)
    })
        .bind(("::", 8080))?
        .run()
        .await?;
    Ok(())
}