use crate::products::DbProduct;

#[derive(Clone, Debug)]
pub struct GetProductResult {
    pub product: DbProduct,
}
