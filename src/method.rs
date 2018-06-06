use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use sede::{self, Decodable, Encodable};

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::mem;
use std::u8;

use result::*;
use types::*;

pub trait Method: Encodable + Decodable {
    fn str_type(param: &str) -> Option<StrType>;

    fn se(&self) -> AmqpResult<Vec<u8>> {
        let mut encoder = Encoder::<Self>::new();
        try!(Encodable::encode(self, &mut encoder));
        try!(encoder.flush_bit());
        Ok(encoder.data)
    }
}

pub fn de<M: Method>(v: &[u8]) -> Result<M, AmqpError> {
    let mut decoder = Decoder::<M>::new(v);
    Decodable::decode(&mut decoder)
}

struct Encoder<M: Method> {
    data: Vec<u8>,
    stack: Vec<Vec<u8>>,
    table_depth: u32,

    bit_mask: u8,
    bit_value: u8,

    is_field_name: bool,
    is_decimal: bool,
    decimal_scale: u8,
    decimal_value: u32,
    is_long_str: bool,
    long_str: Vec<u8>,
    is_timestamp: bool,

    phantom: PhantomData<M>,
}

impl<M: Method> Encoder<M> {
    fn new() -> Encoder<M> {
        Encoder {
            data: Vec::new(),
            stack: Vec::new(),
            table_depth: 0,

            bit_mask: 1,
            bit_value: 0,

            is_field_name: false,
            is_decimal: false,
            decimal_scale: 0,
            decimal_value: 0,
            is_long_str: false,
            long_str: Vec::new(),
            is_timestamp: false,

            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn flush_bit(&mut self) -> AmqpResult<()> {
        if self.bit_mask != 1 {
            self.do_flush_bit()
        } else {
            Ok(())
        }
    }

    #[inline]
    fn do_flush_bit(&mut self) -> AmqpResult<()> {
        try!(self.data.write_u8(self.bit_value));

        self.bit_mask = 1;
        self.bit_value = 0;

        Ok(())
    }

    #[inline]
    fn write_bool(&mut self, v: bool) -> AmqpResult<()> {
        if self.bit_mask == 0 {
            try!(self.do_flush_bit());
        }

        if v {
            self.bit_value |= self.bit_mask;
        }
        self.bit_mask <<= 1;

        Ok(())
    }

    #[inline]
    fn write_u8(&mut self, v: u8) -> AmqpResult<()> {
        try!(self.flush_bit());
        try!(self.data.write_u8(v));
        Ok(())
    }

    #[inline]
    fn write_u16(&mut self, v: u16) -> AmqpResult<()> {
        try!(self.flush_bit());
        try!(self.data.write_u16::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_u32(&mut self, v: u32) -> AmqpResult<()> {
        try!(self.flush_bit());
        try!(self.data.write_u32::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_u64(&mut self, v: u64) -> AmqpResult<()> {
        try!(self.flush_bit());
        try!(self.data.write_u64::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_short_str(&mut self, v: &str) -> AmqpResult<()> {
        try!(self.flush_bit());

        let len = v.len();
        if len > u8::MAX as usize {
            Err(AmqpError::ShortStrTooLong(len))
        } else {
            try!(self.data.write_u8(len as u8));
            try!(self.data.write_all(v.as_bytes()));
            Ok(())
        }
    }

    #[inline]
    fn write_long_str(&mut self) -> AmqpResult<()> {
        try!(self.flush_bit());

        try!(self.data.write_u32::<BigEndian>(self.long_str.len() as u32));
        try!(self.data.write_all(&self.long_str));
        self.long_str.clear();
        Ok(())
    }

    #[inline]
    fn in_table(&self) -> bool {
        self.table_depth > 0
    }

    #[inline]
    fn enter_table(&mut self) -> AmqpResult<()> {
        try!(self.flush_bit());

        self.table_depth += 1;
        self.backup_data();
        Ok(())
    }

    #[inline]
    fn leave_table(&mut self) -> AmqpResult<()> {
        self.table_depth -= 1;

        let mut parent = self.stack.pop().unwrap();
        try!(self.write_table_field_table(&mut parent));
        self.data = parent;
        Ok(())
    }

    #[inline]
    fn enter_array(&mut self) -> AmqpResult<()> {
        if !self.is_long_str {
            try!(self.flush_bit());
            self.backup_data();
        }
        Ok(())
    }

    #[inline]
    fn leave_array(&mut self) -> AmqpResult<()> {
        if self.is_long_str {
            if self.in_table() {
                self.write_table_long_str()
            } else {
                self.write_long_str()
            }
        } else {
            let mut parent = self.stack.pop().unwrap();
            try!(self.write_table_field_array(&mut parent));
            self.data = parent;
            Ok(())
        }
    }

    #[inline]
    fn backup_data(&mut self) {
        self.stack.push(mem::replace(&mut self.data, Vec::new()));
    }

    #[inline]
    fn write_table_field_name(&mut self, v: &str) -> AmqpResult<()> {
        let len = v.len();
        if len > u8::MAX as usize {
            Err(AmqpError::ShortStrTooLong(len))
        } else {
            try!(self.data.write_u8(len as u8));
            try!(self.data.write_all(v.as_bytes()));
            Ok(())
        }
    }

    #[inline]
    fn write_table_bool(&mut self, v: bool) -> AmqpResult<()> {
        try!(self.data.write_u8(b't'));
        try!(self.data.write_u8(v as u8));
        Ok(())
    }

    #[inline]
    fn write_table_i8(&mut self, v: i8) -> AmqpResult<()> {
        try!(self.data.write_u8(b'b'));
        try!(self.data.write_i8(v));
        Ok(())
    }

    #[inline]
    fn write_table_u8(&mut self, v: u8) -> AmqpResult<()> {
        try!(self.data.write_u8(b'B'));
        try!(self.data.write_u8(v));
        Ok(())
    }

    #[inline]
    fn write_table_i16(&mut self, v: i16) -> AmqpResult<()> {
        try!(self.data.write_u8(b'U'));
        try!(self.data.write_i16::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_u16(&mut self, v: u16) -> AmqpResult<()> {
        try!(self.data.write_u8(b'u'));
        try!(self.data.write_u16::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_i32(&mut self, v: i32) -> AmqpResult<()> {
        try!(self.data.write_u8(b'I'));
        try!(self.data.write_i32::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_u32(&mut self, v: u32) -> AmqpResult<()> {
        try!(self.data.write_u8(b'i'));
        try!(self.data.write_u32::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_i64(&mut self, v: i64) -> AmqpResult<()> {
        try!(self.data.write_u8(b'L'));
        try!(self.data.write_i64::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_u64(&mut self, v: u64) -> AmqpResult<()> {
        try!(self.data.write_u8(b'l'));
        try!(self.data.write_u64::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_f32(&mut self, v: f32) -> AmqpResult<()> {
        try!(self.data.write_u8(b'f'));
        try!(self.data.write_f32::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_f64(&mut self, v: f64) -> AmqpResult<()> {
        try!(self.data.write_u8(b'd'));
        try!(self.data.write_f64::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_decimal(&mut self) -> AmqpResult<()> {
        try!(self.data.write_u8(b'D'));
        try!(self.data.write_u8(self.decimal_scale));
        try!(self.data.write_u32::<BigEndian>(self.decimal_value));
        Ok(())
    }

    #[inline]
    fn write_table_short_str(&mut self, v: &str) -> AmqpResult<()> {
        let len = v.len();
        if len > u8::MAX as usize {
            Err(AmqpError::ShortStrTooLong(len))
        } else {
            try!(self.data.write_u8(b's'));
            try!(self.data.write_u8(len as u8));
            try!(self.data.write_all(v.as_bytes()));
            Ok(())
        }
    }

    #[inline]
    fn write_table_long_str(&mut self) -> AmqpResult<()> {
        try!(self.data.write_u8(b'S'));
        try!(self.data.write_u32::<BigEndian>(self.long_str.len() as u32));
        try!(self.data.write_all(&self.long_str));
        self.long_str.clear();
        Ok(())
    }

    #[inline]
    fn write_table_field_array(&mut self, parent: &mut Vec<u8>) -> AmqpResult<()> {
        try!(parent.write_u8(b'A'));
        try!(parent.write_u32::<BigEndian>(self.data.len() as u32));
        try!(parent.write_all(&self.data));
        Ok(())
    }

    #[inline]
    fn write_table_timestamp(&mut self, v: u64) -> AmqpResult<()> {
        try!(self.data.write_u8(b'T'));
        try!(self.data.write_u64::<BigEndian>(v));
        Ok(())
    }

    #[inline]
    fn write_table_field_table(&mut self, parent: &mut Vec<u8>) -> AmqpResult<()> {
        if self.in_table() {
            try!(parent.write_u8(b'F'));
        }
        try!(parent.write_u32::<BigEndian>(self.data.len() as u32));
        try!(parent.write_all(&self.data));
        Ok(())
    }

    #[inline]
    fn write_table_void(&mut self) -> AmqpResult<()> {
        try!(self.data.write_u8(b'V'));
        Ok(())
    }
}

impl<M: Method> sede::Encoder for Encoder<M> {
    type Error = AmqpError;

    #[inline]
    fn emit_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_bool(v)
        } else {
            self.write_bool(v)
        }
    }

    #[inline]
    fn emit_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_i8(v)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn emit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        if self.is_decimal {
            self.decimal_scale = v;
            Ok(())
        } else if self.is_long_str {
            self.long_str.push(v);
            Ok(())
        } else if self.in_table() {
            self.write_table_u8(v)
        } else {
            self.write_u8(v)
        }
    }

    #[inline]
    fn emit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_i16(v)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn emit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_u16(v)
        } else {
            self.write_u16(v)
        }
    }

    #[inline]
    fn emit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_i32(v)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn emit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        if self.is_decimal {
            self.decimal_value = v;
            Ok(())
        } else if self.in_table() {
            self.write_table_u32(v)
        } else {
            self.write_u32(v)
        }
    }

    #[inline]
    fn emit_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_i64(v)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn emit_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        if self.is_timestamp {
            self.write_table_timestamp(v)
        } else if self.in_table() {
            self.write_table_u64(v)
        } else {
            self.write_u64(v)
        }
    }

    #[inline]
    fn emit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_f32(v)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn emit_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        if self.in_table() {
            self.write_table_f64(v)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn emit_str(&mut self, v: &str) -> Result<(), Self::Error> {
        if self.in_table() {
            if self.is_field_name {
                self.write_table_field_name(v)
            } else {
                self.write_table_short_str(v)
            }
        } else {
            self.write_short_str(v)
        }
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_struct<F>(&mut self, name: &str, len: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_struct_field<F>(&mut self, f_name: &str, f_idx: usize, f: F)
    -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        if let Some(StrType::Long) = M::str_type(f_name) {
            self.is_long_str = true;
        }
        let result = f(self);
        self.is_long_str = false;
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_map<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(self.enter_table());
        let result = f(self);
        try!(self.leave_table());
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_map_elt_key<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        self.is_field_name = true;
        let result = f(self);
        self.is_field_name = false;
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_map_elt_val<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_enum<F>(&mut self, name: &str, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_enum_variant<F>(&mut self, v_name: &str, v_id: usize, len: usize, f: F)
    -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        match v_name {
            "DecimalValue" => {
                self.is_decimal = true;
                try!(f(self));
                try!(self.write_table_decimal());
                self.is_decimal = false;
                Ok(())
            },
            "LongString" => {
                self.is_long_str = true;
                try!(f(self));
                self.is_long_str = false;
                Ok(())
            },
            "Timestamp" => {
                self.is_timestamp = true;
                try!(f(self));
                self.is_timestamp = false;
                Ok(())
            },
            _ => {
                if v_name == "Void" {
                    try!(self.write_table_void());
                }
                f(self)
            },
        }
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_enum_variant_arg<F>(&mut self, a_idx: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(self.enter_array());
        let result = f(self);
        try!(self.leave_array());
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_seq_elt<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn emit_nil(&mut self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_char(&mut self, v: char) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_enum_struct_variant<F>(&mut self, v_name: &str, v_id: usize, len: usize, f: F)
    -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_enum_struct_variant_field<F>(&mut self, f_name: &str, f_idx: usize, f: F)
    -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_tuple<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_tuple_arg<F>(&mut self, idx: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_tuple_struct<F>(&mut self, name: &str, len: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_tuple_struct_arg<F>(&mut self, f_idx: usize, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_option<F>(&mut self, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_option_none(&mut self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn emit_option_some<F>(&mut self, f: F) -> Result<(), Self::Error>
    where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        unreachable!()
    }
}

macro_rules! expect {
    ($sf:ident, DecimalValue) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::DecimalValue(scale, value) => {
                $sf.decimal_scale = scale;
                $sf.decimal_value = value;
                Ok(())
            }
            _ => panic!(),
        }
    });
    ($sf:ident, LongString) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::LongString(v) => v,
            _ => panic!(),
        }
    });
    ($sf:ident, FieldArray) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::FieldArray(v) => v,
            _ => panic!(),
        }
    });
    ($sf:ident, Timestamp) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::Timestamp(v) => {
                $sf.timestamp = v;
                Ok(())
            }
            _ => panic!(),
        }
    });
    ($sf:ident, FieldTable) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::FieldTable(v) => v,
            _ => panic!(),
        }
    });
    ($sf:ident, Void) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::Void => Ok(()),
            _ => panic!(),
        }
    });
    ($sf:ident, $t:ident) => ({
        match $sf.table.pop_front().unwrap() {
            FieldValue::$t(v) => Ok(v),
            _ => panic!(),
        }
    })
}

struct Decoder<'a, M: Method> {
    data: &'a [u8],
    table: VecDeque<FieldValue>,
    table_stack: Vec<VecDeque<FieldValue>>,
    table_depth: u32,
    array_len: usize,

    bit_mask: u8,
    bit_value: u8,

    is_decimal: bool,
    decimal_scale: u8,
    decimal_value: u32,
    is_long_str: bool,
    long_str: Vec<u8>,
    is_timestamp: bool,
    timestamp: u64,

    phantom: PhantomData<M>,
}

impl<'a, M: Method> Decoder<'a, M> {
    fn new(data: &'a [u8]) -> Decoder<'a, M> {
        Decoder {
            data: data,
            table: VecDeque::new(),
            table_stack: Vec::new(),
            table_depth: 0,
            array_len: 0,

            bit_mask: 0,
            bit_value: 0,

            is_decimal: false,
            decimal_scale: 0,
            decimal_value: 0,
            is_long_str: false,
            long_str: Vec::new(),
            is_timestamp: false,
            timestamp: 0,

            phantom: PhantomData,
        }
    }

    #[inline]
    fn clear_bit(&mut self) {
        self.bit_mask = 0;
        self.bit_value = 0;
    }

    #[inline]
    fn read_bool(&mut self) -> AmqpResult<bool> {
        if self.bit_mask == 0 {
            self.bit_mask = 1;
            self.bit_value = try!(self.data.read_u8());
        }

        let result = (self.bit_value & self.bit_mask) != 0;
        self.bit_mask <<= 1;
        Ok(result)
    }

    #[inline]
    fn read_u8(&mut self) -> AmqpResult<u8> {
        self.clear_bit();
        Ok(try!(self.data.read_u8()))
    }

    #[inline]
    fn read_u16(&mut self) -> AmqpResult<u16> {
        self.clear_bit();
        Ok(try!(self.data.read_u16::<BigEndian>()))
    }

    #[inline]
    fn read_u32(&mut self) -> AmqpResult<u32> {
        self.clear_bit();
        Ok(try!(self.data.read_u32::<BigEndian>()))
    }

    #[inline]
    fn read_u64(&mut self) -> AmqpResult<u64> {
        self.clear_bit();
        Ok(try!(self.data.read_u64::<BigEndian>()))
    }

    #[inline]
    fn read_short_str(&mut self) -> AmqpResult<String> {
        self.clear_bit();

        let len = try!(self.data.read_u8()) as usize;
        let mut vec = vec![0u8; len];
        try!(self.data.read_exact(&mut vec));
        Ok(try!(String::from_utf8(vec)))
    }

    #[inline]
    fn read_long_str(&mut self) -> AmqpResult<()> {
        self.clear_bit();

        let len = try!(self.data.read_u32::<BigEndian>());
        self.long_str.reserve(len as usize);
        unsafe {
            self.long_str.set_len(len as usize);
        }
        try!(self.data.read_exact(&mut self.long_str));
        self.long_str.reverse();
        Ok(())
    }

    #[inline]
    fn decode_table(&mut self) -> AmqpResult<Table> {
        let mut table = Table::new();
        let table_size = try!(self.data.read_u32::<BigEndian>()) as usize;
        let left = self.data.len() - table_size;

        while self.data.len() > left {
            let field_name_size = try!(self.data.read_u8()) as usize;
            let mut vec = vec![0u8; field_name_size];
            try!(self.data.read_exact(&mut vec));
            let field_name = try!(String::from_utf8(vec));
            let field_value = try!(self.decode_table_field_value());

            table.insert(field_name, field_value);
        }

        Ok(table)
    }

    #[inline]
    fn decode_table_field_value(&mut self) -> AmqpResult<FieldValue> {
        let field_value = match try!(self.data.read_u8()) {
            b't' => FieldValue::Bool(try!(self.data.read_u8()) != 0),
            b'b' => FieldValue::ShortShortInt(try!(self.data.read_i8())),
            b'B' => FieldValue::ShortShortUint(try!(self.data.read_u8())),
            b'U' => FieldValue::ShortInt(try!(self.data.read_i16::<BigEndian>())),
            b'u' => FieldValue::ShortUint(try!(self.data.read_u16::<BigEndian>())),
            b'I' => FieldValue::LongInt(try!(self.data.read_i32::<BigEndian>())),
            b'i' => FieldValue::LongUint(try!(self.data.read_u32::<BigEndian>())),
            b'L' => FieldValue::LongLongInt(try!(self.data.read_i64::<BigEndian>())),
            b'l' => FieldValue::LongLongUint(try!(self.data.read_u64::<BigEndian>())),
            b'f' => FieldValue::Float(try!(self.data.read_f32::<BigEndian>())),
            b'd' => FieldValue::Double(try!(self.data.read_f64::<BigEndian>())),
            b'D' => {
                FieldValue::DecimalValue(try!(self.data.read_u8()),
                                         try!(self.data.read_u32::<BigEndian>()))
            },
            b's' => {
                let len = try!(self.data.read_u8()) as usize;
                let mut vec = vec![0u8; len];
                try!(self.data.read_exact(&mut vec));
                FieldValue::ShortString(try!(String::from_utf8(vec)))
            },
            b'S' => {
                let len = try!(self.data.read_u32::<BigEndian>()) as usize;
                let mut vec = vec![0u8; len];
                try!(self.data.read_exact(&mut vec));
                FieldValue::LongString(vec)
            },
            b'A' => {
                let size = try!(self.data.read_u32::<BigEndian>()) as usize;
                let left = self.data.len() - size;
                let mut array = Vec::new();
                while self.data.len() > left {
                    array.push(try!(self.decode_table_field_value()));
                }
                FieldValue::FieldArray(array)
            },
            b'T' => FieldValue::Timestamp(try!(self.data.read_u64::<BigEndian>())),
            b'F' => FieldValue::FieldTable(try!(self.decode_table())),
            b'V' => FieldValue::Void,
            other => panic!(other),
        };

        Ok(field_value)
    }

    #[inline]
    fn in_table(&self) -> bool {
        self.table_depth > 0
    }

    #[inline]
    fn enter_table(&mut self) -> AmqpResult<()> {
        self.clear_bit();

        if !self.in_table() {
            let table = self.decode_table().unwrap();
            try!(self.flatten_table(table));
        } else {
            try!(self.read_table_field_table());
        }

        self.table_depth += 1;
        Ok(())
    }

    #[inline]
    fn leave_table(&mut self) {
        self.table_depth -= 1;
        self.recover_table()
    }

    #[inline]
    fn enter_array(&mut self) -> AmqpResult<()> {
        self.clear_bit();

        if self.is_long_str {
            if self.in_table() {
                try!(self.read_table_long_str());
            } else {
                try!(self.read_long_str());
            }
            self.array_len = self.long_str.len();
        } else {
            try!(self.read_table_field_array());
            self.array_len = self.table.len();
        }
        Ok(())
    }

    #[inline]
    fn leave_array(&mut self) -> AmqpResult<()> {
        if !self.is_long_str {
            self.recover_table()
        }
        Ok(())
    }

    #[inline]
    fn backup_table(&mut self) {
        self.table_stack.push(mem::replace(&mut self.table, VecDeque::new()));
    }

    #[inline]
    fn recover_table(&mut self) {
        self.table = self.table_stack.pop().unwrap();
    }

    #[inline]
    fn flatten_table(&mut self, table: Table) -> AmqpResult<()> {
        self.backup_table();
        for (k, v) in table {
            self.table.push_back(FieldValue::ShortString(k));
            self.table.push_back(v);
        }
        Ok(())
    }

    #[inline]
    fn flatten_array(&mut self, array: Vec<FieldValue>) -> AmqpResult<()> {
        self.backup_table();
        for v in array {
            self.table.push_back(v);
        }
        Ok(())
    }

    #[inline]
    fn get_table_field_value_name(&self) -> AmqpResult<String> {
        let field_value = self.table.front().unwrap();
        let mut name = format!("{:?}", field_value);
        if let Some(pos) = name.find('(') {
            name.truncate(pos);
        }
        Ok(name)
    }

    #[inline]
    fn read_table_bool(&mut self) -> AmqpResult<bool> {
        expect!(self, Bool)
    }
    #[inline]
    fn read_table_i8(&mut self) -> AmqpResult<i8> {
        expect!(self, ShortShortInt)
    }
    #[inline]
    fn read_table_u8(&mut self) -> AmqpResult<u8> {
        expect!(self, ShortShortUint)
    }
    #[inline]
    fn read_table_i16(&mut self) -> AmqpResult<i16> {
        expect!(self, ShortInt)
    }
    #[inline]
    fn read_table_u16(&mut self) -> AmqpResult<u16> {
        expect!(self, ShortUint)
    }
    #[inline]
    fn read_table_i32(&mut self) -> AmqpResult<i32> {
        expect!(self, LongInt)
    }
    #[inline]
    fn read_table_u32(&mut self) -> AmqpResult<u32> {
        expect!(self, LongUint)
    }
    #[inline]
    fn read_table_i64(&mut self) -> AmqpResult<i64> {
        expect!(self, LongLongInt)
    }
    #[inline]
    fn read_table_u64(&mut self) -> AmqpResult<u64> {
        expect!(self, LongLongUint)
    }
    #[inline]
    fn read_table_f32(&mut self) -> AmqpResult<f32> {
        expect!(self, Float)
    }
    #[inline]
    fn read_table_f64(&mut self) -> AmqpResult<f64> {
        expect!(self, Double)
    }
    #[inline]
    fn read_table_decimal(&mut self) -> AmqpResult<()> {
        expect!(self, DecimalValue)
    }
    #[inline]
    fn read_table_short_str(&mut self) -> AmqpResult<String> {
        expect!(self, ShortString)
    }
    #[inline]
    fn read_table_long_str(&mut self) -> AmqpResult<()> {
        self.long_str = expect!(self, LongString);
        self.long_str.reverse();
        Ok(())
    }
    #[inline]
    fn read_table_field_array(&mut self) -> AmqpResult<()> {
        let array = expect!(self, FieldArray);
        self.flatten_array(array)
    }
    #[inline]
    fn read_table_timestamp(&mut self) -> AmqpResult<()> {
        expect!(self, Timestamp)
    }
    #[inline]
    fn read_table_field_table(&mut self) -> AmqpResult<()> {
        let table = expect!(self, FieldTable);
        self.flatten_table(table)
    }
    #[inline]
    fn read_table_void(&mut self) -> AmqpResult<()> {
        expect!(self, Void)
    }
}

impl<'a, M: Method> sede::Decoder for Decoder<'a, M> {
    type Error = AmqpError;

    #[inline]
    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        if self.in_table() {
            self.read_table_bool()
        } else {
            self.read_bool()
        }
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        if self.in_table() {
            self.read_table_i8()
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        if self.is_decimal {
            Ok(self.decimal_scale)
        } else if self.is_long_str {
            Ok(self.long_str.pop().unwrap())
        } else if self.in_table() {
            self.read_table_u8()
        } else {
            self.read_u8()
        }
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        if self.in_table() {
            self.read_table_i16()
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        if self.in_table() {
            self.read_table_u16()
        } else {
            self.read_u16()
        }
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        if self.in_table() {
            self.read_table_i32()
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        if self.is_decimal {
            Ok(self.decimal_value)
        } else if self.in_table() {
            self.read_table_u32()
        } else {
            self.read_u32()
        }
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        if self.in_table() {
            self.read_table_i64()
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        if self.is_timestamp {
            Ok(self.timestamp)
        } else if self.in_table() {
            self.read_table_u64()
        } else {
            self.read_u64()
        }
    }

    #[inline]
    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        if self.in_table() {
            self.read_table_f32()
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        if self.in_table() {
            self.read_table_f64()
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn read_str(&mut self) -> Result<String, Self::Error> {
        if self.in_table() {
            self.read_table_short_str()
        } else {
            self.read_short_str()
        }
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F)
    -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        if let Some(StrType::Long) = M::str_type(f_name) {
            self.is_long_str = true;
        }
        let result = f(self);
        self.is_long_str = false;
        result
    }

    #[inline]
    fn read_map<T, F>(&mut self, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        try!(self.enter_table());
        let len = self.table.len() / 2;
        let result = f(self, len);
        self.leave_table();
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        let result = f(self);
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_enum<T, F>(&mut self, name: &str, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    #[inline]
    fn read_enum_variant<T, F>(&mut self, names: &[&str], mut f: F) -> Result<T, Self::Error>
    where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> {
        let field_value_name = try!(self.get_table_field_value_name());
        let idx = match names.iter().position(|name| *name == field_value_name) {
            Some(idx) => idx,
            None => unreachable!(),
        };

        match field_value_name.as_ref() {
            "DecimalValue" => {
                self.is_decimal = true;
                try!(self.read_table_decimal());
                let result = f(self, idx);
                self.is_decimal = false;
                result
            },
            "LongString" => {
                self.is_long_str = true;
                let result = f(self, idx);
                self.is_long_str = false;
                result
            },
            "Timestamp" => {
                self.is_timestamp = true;
                try!(self.read_table_timestamp());
                let result = f(self, idx);
                self.is_timestamp = false;
                result
            },
            "Void" => {
                try!(self.read_table_void());
                f(self, idx)
            },
            _ => f(self, idx),
        }
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    #[inline]
    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        try!(self.enter_array());
        let len = self.array_len;
        let result = f(self, len);
        try!(self.leave_array());
        result
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        f(self)
    }

    #[allow(unused_variables)]
    #[inline]
    fn read_nil(&mut self) -> Result<(), Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_isize(&mut self) -> Result<isize, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_char(&mut self) -> Result<char, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T, Self::Error>
    where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F)
    -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_tuple<T, F>(&mut self, len: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F)
    -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn read_option<T, F>(&mut self, f: F) -> Result<T, Self::Error>
    where F: FnMut(&mut Self, bool) -> Result<T, Self::Error> {
        unreachable!()
    }
    #[allow(unused_variables)]
    #[inline]
    fn error(&mut self, err: &str) -> Self::Error {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use protocol::*;
    use super::*;
    use types::*;
    use types::FieldValue::*;

    #[test]
    fn test() {
        let mut a = connection::Start::default();

        let mut table = Table::new();
        table.insert("a".to_string(), Bool(true));
        let nested_table = table.clone();
        table.insert("b".to_string(), FieldArray(vec![FieldTable(nested_table)]));
        let nested_table = table.clone();
        table.insert("c".to_string(), FieldTable(nested_table));
        a.server_properties = table;

        println!("{:#?}", a);

        let vec = a.se().unwrap();
        println!("{:?}", vec);

        let b: connection::Start = de(&vec).unwrap();
        println!("{:#?}", b);

        assert_eq!(a, b);
    }

    #[test]
    fn test_start() {
        let mut a = connection::Start::default();

        let mut table = Table::new();
        table.insert("a".to_string(), Bool(true));
        table.insert("b".to_string(), ShortShortInt(1));
        table.insert("c".to_string(), ShortShortUint(2));
        table.insert("d".to_string(), ShortInt(3));
        table.insert("e".to_string(), ShortUint(4));
        table.insert("f".to_string(), LongInt(5));
        table.insert("g".to_string(), LongUint(6));
        table.insert("h".to_string(), LongLongInt(7));
        table.insert("i".to_string(), LongLongUint(8));
        table.insert("j".to_string(), Float(1.1));
        table.insert("k".to_string(), Double(2.2));
        table.insert("l".to_string(), DecimalValue(1, 2));
        table.insert("m".to_string(), ShortString("short string".to_string()));
        table.insert("n".to_string(), LongString(b"long string".to_vec()));

        let nested_table = table.clone();
        table.insert("o".to_string(),
                     FieldArray(vec![Bool(false),
                                     LongInt(4321),
                                     ShortString("ooxx".to_string()),
                                     FieldTable(nested_table)]));
        table.insert("p".to_string(), Timestamp(1234));
        table.insert("r".to_string(), Void);

        let nested_table = table.clone();
        table.insert("q".to_string(), FieldTable(nested_table));
        a.server_properties = table;

        println!("{:#?}", a);

        let vec = a.se().unwrap();
        println!("{:?}", vec);

        let b: connection::Start = de(&vec).unwrap();
        println!("{:#?}", b);

        assert_eq!(a, b);
    }

    #[test]
    fn test_declare() {
        let mut a = exchange::Declare::default();
        a.passive = true;
        a.durable = false;
        a.auto_delete = true;
        a.internal = false;
        a.nowait = true;

        println!("{:#?}", a);

        let vec = a.se().unwrap();
        println!("{:?}", vec);

        let b: exchange::Declare = de(&vec).unwrap();
        println!("{:#?}", b);

        assert_eq!(a, b);
    }

    #[test]
    fn test_connection_open() {
        let mut a = connection::Open::default();
        println!("{:#?}", a);

        let vec = a.se().unwrap();
        println!("{:?}", vec);

        let b: connection::Open = de(&vec).unwrap();
        println!("{:#?}", b);

        assert_eq!(a, b);
    }
}
