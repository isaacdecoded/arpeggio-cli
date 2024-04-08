use std::any::Any;
use chrono::{ Local, DateTime };

pub trait DomainEvent: Send + Sync {
    fn get_name(&self) -> String;
    fn get_aggregate_root_id(&self) -> &String;
    fn get_occurring_time(&self) -> &DateTime<Local>;
    fn as_any(&self) -> &dyn Any;
}
