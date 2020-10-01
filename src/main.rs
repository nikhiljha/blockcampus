use serde::{Deserialize, Serialize};
use actix_files::Files;
use actix_web::{web, App, post, get, HttpResponse, HttpRequest, HttpServer, Responder, error, Error};
use tera::Tera;
use std::env::temp_dir;
use lettre::file::FileTransport;
use lettre::{SendableEmail, Envelope, EmailAddress, Transport};

#[derive(Serialize, Deserialize)]
pub struct RegistrationParams {
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LinkParams {
    uuid: String,
}

#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s :String = tmpl.render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/account/register")]
async fn register_post(tmpl: web::Data<tera::Tera>, params: web::Form<RegistrationParams>) -> Result<HttpResponse, Error> {
    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new("admin@blocks.isogram.org".to_string()).unwrap()),
            vec![EmailAddress::new(params.email.clone()).unwrap()],
        ).unwrap(),
        "id".to_string(),
        "Hey, click this link.".to_string().into_bytes(),
    );
    println!("{:?}", temp_dir());
    let mut sender = FileTransport::new(temp_dir());
    let result = sender.send(email);
    if result.is_err() {
        return Ok(HttpResponse::Ok().content_type("text/html").body("<p>mail error</p>"))
    }
    assert!(result.is_ok());

    let mut ctx = tera::Context::new();
    ctx.insert("email", &params.email);
    let s = tmpl.render("account/sent_email.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/account/register")]
async fn register_get(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s :String = tmpl.render("account/register.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/account/link/{token}")]
async fn link_post(req: HttpRequest, params: web::Form<LinkParams>) -> Result<HttpResponse, Error> {
    // TODO: Link the account.
    let token = req.match_info().get("token").unwrap_or("000");
    Ok(HttpResponse::Ok().content_type("text/html").body(format!("<p>Linked UUID {} with token {}!</p>", &params.uuid, &token)))
}

#[get("/account/link/{token}")]
async fn link_get(tmpl: web::Data<tera::Tera>, req: HttpRequest) -> Result<HttpResponse, Error> {
    // TODO: Verify if token is valid.
    let s :String = tmpl.render("account/link.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(tera)
            .service(link_get)
            .service(link_post)
            .service(register_get)
            .service(register_post)
            .service(Files::new("/svelte/", "svelte/public/"))
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
