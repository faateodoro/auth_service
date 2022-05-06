#![allow(dead_code, unused_variables)]

struct Credentials {
    username: String,
    password: String
}

enum Status {
    Connected,
    Interrupted
}

fn connect_to_database() -> Status {
    Status::Connected
}

fn login(creds: Credentials) {
    // authenticate
    get_user();
}

fn get_user() {
    // get user from database
}

fn logout() {
    // log user out
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}