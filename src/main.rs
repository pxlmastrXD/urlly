mod pixeldb;

use pixeldb::Database;

fn main() {
    let db = Database{db: "./database"};
    db.set(key: "key", value: "value");
    println!("{}", db.get("key"))
}