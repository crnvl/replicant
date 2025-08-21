pub fn check_valid_username(username: &str) -> bool {
    // usernames can only contain alphanumeric characters with a maximum length of 20 characters
    username.len() <= 20 && username.chars().all(char::is_alphanumeric)
}
