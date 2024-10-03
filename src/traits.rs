use std::collections::HashSet;

use crate::prelude::FieldHeader;





pub trait UnhandledField {
    fn get_unhandled_field(&self) -> Option<FieldHeader>;
}

pub trait UnhandledFields {
    fn get_unhandled_fields(&self) -> HashSet<FieldHeader>;
}