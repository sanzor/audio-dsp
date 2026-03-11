use crate::products::DbProduct;

#[derive(Clone, Debug)]
pub struct CreateProductResult {
    pub product: DbProduct,
}
