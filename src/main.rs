use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use once_cell::sync::Lazy;

mod pixeldb;
use pixeldb::Database;

mod minibash;

static DB: Lazy<Database> = Lazy::new(|| Database { db: "./db" });

/// Create new shortened link
fn newLink(db: &Database, link: &str) -> String {
    // Ensure keynumber file exists
    let _ = minibash::runbash("touch db/keynumber");

    // Read current key number
    let prevkeynum = minibash::runbash("cat db/keynumber");

    if !minibash::checkOutput(prevkeynum.clone()) {
        panic!("Failed to read key number");
    }

    // Convert to integer
    let keynumint: i32 = prevkeynum.trim().parse().expect("Failed to parse key number") + 1;
    let keynum = keynumint.to_string();

    // Store link in database
    let key = db.set(&keynum, link);

    // Update keynumber file
    let _ = minibash::runbash(format!("echo {keynum} > db/keynumber"));

    key
}

/// Retrieve link from database
async fn getLink(db: &Database, key: &str) -> String {
    db.get(key)
}

/// Route to create a link
#[post("/createLink")]
async fn create_link_handler(reqbody: String) -> impl Responder {
    let key = newLink(&DB, &reqbody);
    HttpResponse::Ok().body(key)
}

/// Route to retrieve a link
#[get("/getLink/{key}")]
async fn get_link_handler(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    let link = getLink(&DB, &key).await;
    HttpResponse::Ok().body(link)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_link_handler)
            .service(get_link_handler)
            .service(fs::Files::new("/", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
