use rust_decimal::Decimal;
pub type ClientId = u64;

#[derive(Clone, Debug)]
pub struct Client {
    pub id: ClientId,
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
    pub balance: Decimal,
}
