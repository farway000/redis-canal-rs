#![allow(unused_must_use)]

use super::write_str;
use crate::formatter::Formatter;
use crate::types::EncodingType;
use serialize::Result;
use std::io;
use std::io::Write;
use std::str;

pub struct JSON {
    out: Box<dyn Write + 'static>,
    is_first_db: bool,
    has_databases: bool,
    is_first_key_in_db: bool,
    elements_in_key: u64,
    element_index: u64,
}

impl JSON {
    pub fn new() -> JSON {
        let out = Box::new(io::stdout());
        JSON {
            out: out,
            is_first_db: true,
            has_databases: false,
            is_first_key_in_db: true,
            elements_in_key: 0,
            element_index: 0,
        }
    }
}

fn encode_to_ascii(value: &[u8]) -> String {
    let s = unsafe { str::from_utf8_unchecked(value) };
    let result = serialize::to_string(&s);
    let _result = match result {
        Ok(a) => a,
        Err(error) => panic!("encode_to_ascii error: {:?}", error),
    };
    _result
}

impl JSON {
    fn start_key(&mut self, length: u64) {
        if !self.is_first_key_in_db {
            write_str(&mut self.out, ",");
        }

        self.is_first_key_in_db = false;
        self.elements_in_key = length;
        self.element_index = 0;
    }

    fn end_key(&mut self) {}

    fn write_comma(&mut self) {
        if self.element_index > 0 {
            write_str(&mut self.out, ",");
        }
        self.element_index += 1;
    }

    fn write_key(&mut self, key: &[u8]) {
        self.out.write_all(encode_to_ascii(key).as_bytes());
    }
    fn write_value(&mut self, value: &[u8]) {
        self.out.write_all(encode_to_ascii(value).as_bytes());
    }
}

impl Formatter for JSON {
    fn start_rdb(&mut self) {
        write_str(&mut self.out, "[");
    }

    fn end_rdb(&mut self) {
        if self.has_databases {
            write_str(&mut self.out, "}");
        }
        write_str(&mut self.out, "]\n");
    }

    fn start_database(&mut self, _db_number: u64) {
        if !self.is_first_db {
            write_str(&mut self.out, "},");
        }

        write_str(&mut self.out, "{");
        self.is_first_db = false;
        self.has_databases = true;
        self.is_first_key_in_db = true;
    }

    fn set(&mut self, key: &[u8], value: &[u8], _expiry: Option<u64>) {
        self.start_key(0);
        self.write_key(key);
        write_str(&mut self.out, ":");
        self.write_value(value);
    }

    fn start_hash(&mut self, key: &[u8], length: u64, _expiry: Option<u64>, _info: EncodingType) {
        self.start_key(length);
        self.write_key(key);
        write_str(&mut self.out, ":{");
        self.out.flush();
    }

    fn end_hash(&mut self, _key: &[u8]) {
        self.end_key();
        write_str(&mut self.out, "}");
        self.out.flush();
    }

    fn hash_element(&mut self, _key: &[u8], field: &[u8], value: &[u8]) {
        self.write_comma();
        self.write_key(field);
        write_str(&mut self.out, ":");
        self.write_value(value);
        self.out.flush();
    }

    fn start_set(
        &mut self,
        key: &[u8],
        cardinality: u64,
        _expiry: Option<u64>,
        _info: EncodingType,
    ) {
        self.start_key(cardinality);
        self.write_key(key);
        write_str(&mut self.out, ":[");
        self.out.flush();
    }

    fn end_set(&mut self, _key: &[u8]) {
        self.end_key();
        write_str(&mut self.out, "]");
    }

    fn set_element(&mut self, _key: &[u8], member: &[u8]) {
        self.write_comma();
        self.write_value(member);
    }

    fn start_list(&mut self, key: &[u8], length: u64, _expiry: Option<u64>, _info: EncodingType) {
        self.start_key(length);
        self.write_key(key);
        write_str(&mut self.out, ":[");
    }

    fn end_list(&mut self, _key: &[u8]) {
        self.end_key();
        write_str(&mut self.out, "]");
    }

    fn list_element(&mut self, _key: &[u8], value: &[u8]) {
        self.write_comma();
        self.write_value(value);
    }

    fn start_sorted_set(
        &mut self,
        key: &[u8],
        length: u64,
        _expiry: Option<u64>,
        _info: EncodingType,
    ) {
        self.start_key(length);
        self.write_key(key);
        write_str(&mut self.out, ":{");
    }

    fn end_sorted_set(&mut self, _key: &[u8]) {
        self.end_key();
        write_str(&mut self.out, "}");
    }

    fn sorted_set_element(&mut self, _key: &[u8], score: f64, member: &[u8]) {
        self.write_comma();
        self.write_key(member);
        write_str(&mut self.out, ":");
        self.write_value(score.to_string().as_bytes());
    }
}
