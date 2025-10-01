pub mod wasm;
pub mod audio;
pub mod db;

fn main() {
    db::database::pet_shop_db();
}

