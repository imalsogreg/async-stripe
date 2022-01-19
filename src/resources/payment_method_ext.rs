use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::resources::{
    CreatePaymentMethod, PaymentMethod, PaymentMethodTypeFilter, UpdatePaymentMethod,
};

/// The parameters for `PaymentMethod::attach`
///
/// For more details see <https://stripe.com/docs/api/payment_methods/attach>.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttachPaymentMethod {
    pub customer: CustomerId,
}

#[derive(Clone, Debug, Serialize)]
pub struct PaymentCard {
    pub exp_year: u16,
    pub exp_month: u8,
    pub number: String,
    pub cvc: u16,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreatePaymentMethodWithCard<'a> {
    #[serde(flatten)]
    pub create_payment_method: CreatePaymentMethod<'a>,
    pub card: PaymentCard,
}

#[derive(Clone, Debug, Serialize)]
pub struct UpdatePaymentMethodWithCard<'a> {
    #[serde(flatten)]
    pub update_payment_method: UpdatePaymentMethod<'a>,
    pub card: PaymentCard,
}

impl PaymentMethod {
    /// Attach a payment method to a customer
    ///
    /// For more details see <https://stripe.com/docs/api/payment_methods/attach>.
    pub fn attach(
        client: &Client,
        payment_method_id: &PaymentMethodId,
        params: AttachPaymentMethod,
    ) -> Response<PaymentMethod> {
        client.post_form(&format!("/payment_methods/{}/attach", payment_method_id), params)
    }

    /// Detach a PaymentMethod from a Customer
    ///
    /// For more details see <https://stripe.com/docs/api/payment_methods/detach>.
    pub fn detach(client: &Client, payment_method_id: &PaymentMethodId) -> Response<PaymentMethod> {
        client.post(&format!("/payment_methods/{}/detach", payment_method_id))
    }

    /// Create a payment method with a card.
    pub fn create_with_card<'a>(
        client: &Client,
        mut params: CreatePaymentMethodWithCard<'a>,
    ) -> Response<PaymentMethod> {
        params.create_payment_method.type_ = Some(PaymentMethodTypeFilter::Card);
        client.post_form("/payment_methods", params)
    }

    /// Create a payment method with a card.
    pub fn update_with_card<'a>(
        client: &Client,
        payment_method_id: &PaymentMethodId,
        params: UpdatePaymentMethodWithCard<'a>,
    ) -> Response<PaymentMethod> {
        client.post_form(format!("/payment_methods/{}", payment_method_id).as_str(), params)
    }
}
