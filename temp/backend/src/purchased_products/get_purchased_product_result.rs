use crate::purchased_products::DbPurchasedProduct;

#[derive(Clone, Debug)]
pub struct GetPurchasedProductResult {
    pub purchased_product: DbPurchasedProduct,
}
