
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub key: String,
    pub vault: String,
}