use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse} };
use actix_web::middleware::Next;

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    todo!()
}
