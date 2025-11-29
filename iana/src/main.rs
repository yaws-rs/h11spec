//! Generate required iana types

use std::fs::File;
use std::io::Write;
use serde::Deserialize;
use regex::Regex;

//https://www.iana.org/assignments/http-fields/field-names.csv

//Field Name,Status,Structured Type,Reference,Comments

#[derive(Debug, Deserialize)]
struct Field {
    #[serde(rename = "Field Name")]
    field_name: String,
    #[serde(rename = "Support")]
    support: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Structured Type")]
    structured_type: String,
    #[serde(rename ="Reference")]
    reference: String,
    #[serde(rename ="Comments")]
    comments: String,
}

fn generate_field_names() {
    let mut file = File::open("data/field-names-2025-11-28.csv").unwrap();

    let mut out_tokens_file = File::create("../h11types/src/generated/h11header_name_tokens.rs").unwrap();
    let mut out_name_file = File::create("../h11types/src/generated/h11header_name.rs").unwrap();
    let mut out_impl_file = File::create("../h11types/src/generated/h11header_impl.rs").unwrap();
    
    let mut rdr = csv::Reader::from_reader(file);
    out_tokens_file.write(b"//! (auto generated from iana registry) h11Header Name tokens\n\nuse logos::{Lexer, Logos};\n\n#[derive(Debug, Logos)]\n#[logos(source = [u8])]\npub(crate) enum HeaderKeyToken<'raw> {\n").unwrap();
    out_name_file.write(b"//! (auto generated from iana registry) h11Header Name\n\n#[derive(Debug, PartialEq)]\npub enum HeaderKey {\n").unwrap();

    out_impl_file.write(b"//! (auto generated from iana registry) h11header Token -> Name impl\n\nimpl TryFrom<HeaderKeyToken<'_>> for HeaderKey {\n    type Error = crate::H11Error;\n    fn try_from(i: HeaderKeyToken<'_>) -> Result<Self, Self::Error> {\n        match i {\n").unwrap();
    
    let enum_re_sanitizer = Regex::new(r"[^A-Za-z0-9]").unwrap();
    
    for result in rdr.deserialize() {
        let field: Field = result.unwrap();
        let enum_variant = enum_re_sanitizer.replace_all(&field.field_name, "");

        if field.support == "Skip" {
            continue;
        }
        
        if enum_variant == "" {
            continue;
        }
        
        out_tokens_file.write(format!("    #[regex(r\"(?i:{}):\\s*([^\\r\\n]*)\\r\\n\", crate::generated::util::header_value_u8)]\n    {}(&'raw [u8]),\n", field.field_name, enum_variant).as_bytes()).unwrap();
        out_name_file.write(format!("    {},\n", enum_variant).as_bytes()).unwrap();
        out_impl_file.write(format!("            HeaderKeyToken::{} => Ok(Self::{}),\n", enum_variant, enum_variant).as_bytes());
    }

    out_tokens_file.write(b"    #[allow(dead_code)]\n    Phantom(&'raw [u8]),\n").unwrap();
    out_impl_file.write(b"             _ => Err(H11Error::MissingHeaderKey),\n").unwrap();
    
    out_name_file.write(b"}\n").unwrap();
    out_tokens_file.write(b"}\n").unwrap();
    out_impl_file.write(b"         }\n    }\n}\n").unwrap();
    out_name_file.flush().unwrap();
    out_tokens_file.flush().unwrap();
    out_impl_file.flush().unwrap();
}


fn main() {
    generate_field_names();
}
