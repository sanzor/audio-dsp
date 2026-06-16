use crate::domain_user::UserId;

pub type TicketId=i64;
pub struct DbTicket{
    pub id:TicketId,
    pub issued_by:UserId,
    pub timestamp:i64
}