/// Importing of neccessary modules and libraries
// PixelDB (Backend Database)
mod pixeldb;
use pixeldb::Database;

// Minibash (for filesystem operations using bash)
mod minibash;

// Actix-web (HTTP Handling)
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/// Database operations
fn newLink(db: Database, link: &str) {
    // Key number operation
    // Create keynum (first run) / touch it to make sure
    let _ = minibash::runbash("touch db/keynumber");
    // get keynum and check
    let prevkeynum = minibash::runbash("cat db/keynumber");
    match minibash::checkOutput(prevkeynum) {
        true => _,
        false => panic!("Failed to read key number")
    };
    let mut keynumint: i32 = prevkeynum.parse().unwrap() + 1;
    let keynum = keynumint.to_string()

    /// Set key and handle set errors
    // set key
    let key = db.set(keynum, link);

    // increment keynum
    let _ = minibash::runbash(format!("echo {key} > db/keynumber"));
    
    return key
}





fn main() {
    
}