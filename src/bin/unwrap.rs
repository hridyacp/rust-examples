fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    let user_option = get_user("Hari");

    let result = match user_option {
        Some(user) => user,
        None => "not found!",
    };

    println!("user = {:?}", result);
}