#[derive(Default, sqlx::FromRow)]
pub struct Flight {
    pub id: i32,
    pub tail: String,
    pub from: String,
    pub to: String,
    pub etd: String,
    pub eta: String,
    pub flight_number: String,
}
