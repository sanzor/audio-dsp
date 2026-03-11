use crate::products::DbProduct;

#[derive(Clone, Debug)]
pub struct UpdateProductResult {
    pub product: DbProduct,
}
