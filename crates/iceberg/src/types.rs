#[derive(PartialEq, Eq)]
pub enum FieldType {
    Boolean,
    Integer,
    Long,
    Float,
    Double,
    Date,
    Time,
    Timestamp,
    String,
    Uuid,
    Fixed,
    Binary,
    Decimal,
    Struct,
    List,
    Map
}

pub type Boolean = bool;
pub type Integer = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;