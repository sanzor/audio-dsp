use async_trait::async_trait;

use crate::checkout::checkout_error::CheckoutError;
use crate::checkout::create_subscription_checkout_params::CreateSubscriptionCheckoutParams;
use crate::checkout::create_subscription_checkout_result::CreateSubscriptionCheckoutResult;
use crate::checkout::create_token_pack_checkout_params::CreateTokenPackCheckoutParams;
use crate::checkout::create_token_pack_checkout_result::CreateTokenPackCheckoutResult;

#[async_trait]
pub trait CheckoutProvider: Send + Sync {
    async fn create_token_pack_checkout(
        &self,
        params: CreateTokenPackCheckoutParams,
    ) -> Result<CreateTokenPackCheckoutResult, CheckoutError>;

    async fn create_subscription_checkout(
        &self,
        params: CreateSubscriptionCheckoutParams,
    ) -> Result<CreateSubscriptionCheckoutResult, CheckoutError>;
}
