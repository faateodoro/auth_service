#![allow(dead_code, unused_variables)]

mod database {
    enum Status {
        Connected,
        Interrupted
    }

    fn connect_to_database() -> Status {
        Status::Connected
    }

    fn get_user() {
        // get user from database
    }
}

mod auth_utils {
    pub fn login(creds: Credentials) {
        // authenticate
        get_user();
    }
    
    fn logout() {
        // log user out
    }

    mod models{
        pub struct Credentials {
            username: String,
            password: String
        }
    }
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}