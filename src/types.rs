use std::collections::HashMap;

pub enum StrType {
    Short,
    Long,
}

pub type Bit = bool;
pub type Octet = u8;
pub type Short = u16;
pub type Long = u32;
pub type Longlong = u64;
pub type Shortstr = String;
pub type Longstr = Vec<u8>;

pub type FieldName = Shortstr;
pub type Table = HashMap<FieldName, FieldValue>;

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum FieldValue {
    Bool(bool),
    ShortShortInt(i8),
    ShortShortUint(u8),
    ShortInt(i16),
    ShortUint(u16),
    LongInt(i32),
    LongUint(u32),
    LongLongInt(i64),
    LongLongUint(u64),
    Float(f32),
    Double(f64),
    DecimalValue(u8, u32),
    ShortString(Shortstr),
    LongString(Longstr),
    FieldArray(Vec<FieldValue>),
    Timestamp(u64),
    FieldTable(Table),
    Void,
}

pub fn to_string(s: Longstr) -> String {
    unsafe {
        String::from_utf8_unchecked(s)
    }
}
