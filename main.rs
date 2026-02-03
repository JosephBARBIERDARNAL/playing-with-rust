struct User {
    username: String,
    sign_in_count: u32,
    active: bool,
}

impl User {
    fn new(username: String) -> User {
        User {
            username,
            sign_in_count: 0,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn is_active(&self) -> bool {
        self.active
    }
}

fn main() {
    let mut jo = User::new("Joseph".to_string());
    println!("{}", jo.is_active());
    jo.deactivate();
    println!("{}", jo.is_active());
}
