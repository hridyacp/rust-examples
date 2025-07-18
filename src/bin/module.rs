pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create");
        }
    }
}

use player::sprite::create;

fn main() {
    create();
}