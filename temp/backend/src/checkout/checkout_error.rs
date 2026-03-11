pub enum CheckoutError {
    NotFound(String),
    InvalidInput(String),
    Internal(String),
}
