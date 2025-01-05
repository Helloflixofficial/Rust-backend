pub mod hello_user;
pub use hello_user::*;

pub mod home;
pub use home::*;

pub mod todos;
// #[allow(unused_imports)]
pub use todos::*;

pub mod create_user;
pub use create_user::*;

fn logging(path: &str) {
    println!("Path accessed: {}", path);
}
