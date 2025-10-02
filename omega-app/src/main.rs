pub mod audio;
pub mod db;
pub mod wasm;

fn main() {
    db::database::pet_shop_db();
}
