extern crate rustc_serialize as sede;
extern crate zadt;
#[macro_use]
extern crate zbase;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use std::u8;

use sede::json::Json;

use zadt::zstr;
use zbase::zenv;
use zbase::zproc;

const HEADER_STRUCT: &'static str = r#"
#[derive(Debug)]
pub struct Header {
    name: &'static [u8; 4],
    major: u8,
    minor: u8,
    revision: u8,
}
"#;

const DEFAULT_CALL: &'static str = "Default::default()";

macro_rules! src {
    (HEADER) => (r#"
pub const HEADER: Header = Header {{
    name: b"{}",
    major: {},
    minor: {},
    revision: {},
}};
"#);

    (PORT) => (r#"
pub const PORT: u16 = {};
"#);

    (PROT_HEADER) => (r#"
pub const PROT_HEADER: [u8; 8] = [{}, {}, {}, {}, 0, {}, {}, {}];
"#);

    (CONST_U8) => (r#"
pub const {}: u8 = {};"#);
    (CONST_U16) => (r#"
pub const {}: u16 = {};"#);

    (CLASS_MOD) => (r#"
pub mod {} {{
    use method::Method;
    use types::*;
    {}
}}
"#);

    (METHOD_STRUCT) => (r#"
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct {} {{
        cid: Short,
        id: Short,{}
    }}
"#);

    (METHOD_STRUCT_FIELD) => (r#"
        pub {}: {},"#);

    (METHOD_IMPL) => (r#"
    impl Method for {} {{{}
    }}
"#);

    (METHOD_IMPL_STR_TYPE) => (r#"
        fn str_type(param: &str) -> Option<StrType> {{
            match param {{ {}
                _ => None
            }}
        }}"#);

    (STR_TYPE_SHORT_STR) => (r#"
                "{}" => Some(StrType::Short),"#);

    (STR_TYPE_LONG_STR) => (r#"
                "{}" => Some(StrType::Long),"#);

    (METHOD_DEFAULT) => (r#"
    impl Default for {} {{
        fn default() -> {} {{
            {} {{
                cid: {},
                id: {},{}
            }}
        }}
    }}
"#);

    (METHOD_DEFAULT_FIELD) => (r#"
                {}: {},"#);
}

macro_rules! fmt_src {
    ($src:tt, $($value:tt)*) => (format!(src!($src), $($value)*));
}

fn usage() {
    pe!("Usage: {} <amqp_protocol_spec.json> <protocol.rs>", zenv::bin_name().unwrap().unwrap());
}

fn gen_prot(json_path: String, rs_path: String) {
    let mut json_file = File::open(json_path).unwrap();
    let json = Json::from_reader(&mut json_file).unwrap();

    let mut rs = String::new();
    rs.push_str(&gen_header(&json));
    rs.push_str(&gen_consts(&json));
    rs.push('\n');
    rs.push_str(&gen_classes(&json));

    let mut rs_file = File::create(rs_path).unwrap();
    rs_file.write_all(&rs.as_bytes()[1..]).unwrap();
}

fn gen_header(json: &Json) -> String {
    let mut s = String::new();
    s.push_str(HEADER_STRUCT);

    let name = json["name"].as_string().unwrap();
    let header = fmt_src!(HEADER, name,
                          json["major-version"], json["minor-version"], json["revision"]);
    s.push_str(&header);

    let port = fmt_src!(PORT, json["port"]);
    s.push_str(&port);

    let name_bytes = name.as_bytes();
    let prot_header = fmt_src!(PROT_HEADER,
                               name_bytes[0], name_bytes[1], name_bytes[2], name_bytes[3],
                               json["major-version"], json["minor-version"], json["revision"]);
    s.push_str(&prot_header);

    s
}

fn gen_consts(json: &Json) -> String {
    json["constants"].as_array().unwrap().iter().map(|c| {
        let name = zstr::hyphen_to_snake(c["name"].as_string().unwrap());
        let value = c["value"].as_i64().unwrap();
        if value > u8::MAX as i64 {
            fmt_src!(CONST_U16, name, value as u16)
        } else {
            fmt_src!(CONST_U8, name, value as u8)
        }
    }).collect()
}

fn gen_classes(json: &Json) -> String {
    let domains = get_domains(json);
    json["classes"].as_array().unwrap().iter().map(|class| {
        gen_class(class, &domains)
    }).collect()
}

fn get_domains(json: &Json) -> HashMap<&str, &str> {
    json["domains"].as_array().unwrap().iter().map(|e| {
        let kv = e.as_array().unwrap();
        (kv[0].as_string().unwrap(), kv[1].as_string().unwrap())
    }).collect()
}

fn gen_class(class: &Json, domains: &HashMap<&str, &str>) -> String {
    let class_name = class["name"].as_string().unwrap();
    let mut methods = gen_methods(class, domains);
    let len = methods.len();
    methods.truncate(len - 1);
    fmt_src!(CLASS_MOD, class_name, methods)
}

fn gen_methods(class: &Json, domains: &HashMap<&str, &str>) -> String {
    class["methods"].as_array().unwrap().iter().map(|method| {
        gen_method(class, method, domains)
    }).collect()
}

fn gen_method(class: &Json, method: &Json, domains: &HashMap<&str, &str>) -> String {
    let mut s = String::new();
    s.push_str(&gen_method_struct(method, domains));
    s.push_str(&gen_method_impl(method));
    s.push_str(&gen_method_default(class, method));
    s
}

fn gen_method_struct(method: &Json, domains: &HashMap<&str, &str>) -> String {
    let name = zstr::hyphen_to_camel(method["name"].as_string().unwrap());
    let fields = gen_method_struct_fields(method, domains);
    fmt_src!(METHOD_STRUCT, name, fields)
}

fn gen_method_struct_fields(method: &Json, domains: &HashMap<&str, &str>) -> String {
    method["arguments"].as_array().unwrap().iter().map(|arg| {
        gen_method_struct_field(arg, domains)
    }).collect()
}

fn gen_method_struct_field(arg: &Json, domains: &HashMap<&str, &str>) -> String {
    let name = get_field_name(arg);
    let value = match arg.find("type") {
        Some(v) => v.as_string().unwrap(),
        None => domains[arg["domain"].as_string().unwrap()],
    };
    let ty = zstr::hyphen_to_camel(value);

    fmt_src!(METHOD_STRUCT_FIELD, name, ty)
}

fn gen_method_impl(method: &Json) -> String {
    let name = zstr::hyphen_to_camel(method["name"].as_string().unwrap());
    let impl_str_type = gen_method_impl_str_type(method);
    fmt_src!(METHOD_IMPL, name, impl_str_type)
}

fn gen_method_impl_str_type(method: &Json) -> String {
    let mut patterns = String::new();

    let args = method["arguments"].as_array().unwrap();
    for arg in args {
        let name = get_field_name(arg);
        if let Some(ty) = arg.find("type") {
            match ty.as_string().unwrap() {
                "shortstr" => patterns.push_str(&fmt_src!(STR_TYPE_SHORT_STR, name)),
                "longstr" => patterns.push_str(&fmt_src!(STR_TYPE_LONG_STR, name)),
                _ => continue,
            }
        }
    }
    fmt_src!(METHOD_IMPL_STR_TYPE, patterns)
}

fn gen_method_default(class: &Json, method: &Json) -> String {
    let name = zstr::hyphen_to_camel(method["name"].as_string().unwrap());
    let cid = class["id"].as_u64().unwrap();
    let id = method["id"].as_u64().unwrap();
    let fields = gen_method_default_fields(method);
    fmt_src!(METHOD_DEFAULT, name, name, name, cid, id, fields)
}

fn gen_method_default_fields(method: &Json) -> String {
    method["arguments"].as_array().unwrap().iter().map(|arg| {
        let name = get_field_name(arg);
        let value = match arg.find("default-value") {
            Some(json) => {
                let ty = match arg.find("type") {
                    Some(ty_json) => ty_json.as_string().unwrap(),
                    None => arg.find("domain").unwrap().as_string().unwrap(),
                };
                get_field_value(json, ty)
            },
            None => DEFAULT_CALL.to_string(),
        };

        fmt_src!(METHOD_DEFAULT_FIELD, name, value)
    }).collect()
}

fn get_field_name(arg: &Json) -> String {
    let mut name = zstr::hyphen_to_snake(arg["name"].as_string().unwrap());
    if name == "type" {
        name = "ty".to_string();
    }
    name
}

fn get_field_value(json: &Json, ty: &str) -> String {
    match json {
        &Json::String(_) => {
            match ty {
                "shortstr" => get_short_str_value(json),
                "longstr" => get_long_str_value(json),
                _ => panic!("{}", ty),
            }
        },
        &Json::Object(_) => get_object_value(json),
        _ => get_other_value(json),
    }
}

fn get_short_str_value(json: &Json) -> String {
    format!("{}.to_string()", json)
}

fn get_long_str_value(json: &Json) -> String {
    format!("b{}.to_vec()", json)
}

fn get_object_value(json: &Json) -> String {
    if json.as_object().unwrap().is_empty() {
        DEFAULT_CALL.to_string()
    } else {
        panic!("{}", json)
    }
}

fn get_other_value(json: &Json) -> String {
    format!("{}", json)
}

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        usage();
        process::exit(zproc::EXIT_ERR);
    }

    args.next();
    gen_prot(args.next().unwrap(), args.next().unwrap());
}
