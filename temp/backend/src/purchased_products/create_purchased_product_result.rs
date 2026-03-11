use crate::purchased_products::DbPurchasedProduct;

#[derive(Clone, Debug)]
pub struct CreatePurchasedProductResult {
    pub purchased_product: DbPurchasedProduct,
}
