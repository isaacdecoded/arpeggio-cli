use chrono::{ Local, DateTime };
use crate::core::domain::models::value_object::ValueObject;

#[derive(Copy, Clone, PartialEq)]
pub struct DateValueObject {
    value: DateTime<Local>,
}

impl ValueObject<DateTime<Local>> for DateValueObject {
    fn new(value: DateTime<Local>) -> Self {
        DateValueObject { value }
    }

    fn get_value(&self) -> &DateTime<Local> {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value == *other.get_value()
    }
}

impl DateValueObject {
    pub fn _now() -> Self {
        DateValueObject::new(Local::now())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use crate::core::domain::models::{
        value_object::ValueObject,
        date_value_object::DateValueObject,
    };

    #[test]
    fn should_initialize_valid_instance() {
        let value = Local::now();
        let vo = DateValueObject::new(value);
        assert_eq!(vo.get_value().to_owned(), value);
    }
}
