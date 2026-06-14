pub struct TicketCreated{
    id:TicketId,
}
pub struct TicketStatusResult{
    id:TicketId,
    status:TicketStatus
}
pub enum TicketStatus{
    Processing,
    Failed,
    Successful{resource_id:ResourceId}
}
pub type TicketId=i64;
pub type ResourceId=i64;