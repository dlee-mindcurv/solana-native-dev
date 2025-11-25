pub enum User {
    createPda(UserInfo),
    sayHello(UserInfo),
}

#[derive(Debug)]
pub struct UserInfo {
    name: String,
    age: u8,
}
