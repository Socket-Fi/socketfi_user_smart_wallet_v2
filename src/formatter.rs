use soroban_sdk::{
    xdr::{FromXdr, ToXdr},
    Bytes, Env, String,
};

pub fn to_lower_bytes(e: &Env, string: String) -> Bytes {
    let string_xdr = string.clone().to_xdr(e);

    let mut formatted_string_xdr = string_xdr.clone();

    for i in 0..string_xdr.len() {
        let ascii_val = string_xdr.get_unchecked(i);
        if ascii_val >= 65 && ascii_val <= 90 {
            formatted_string_xdr.set(i, ascii_val + 32);
        }
    }

    formatted_string_xdr
}

pub fn to_upper_bytes(e: &Env, string: String) -> Bytes {
    let string_xdr = string.clone().to_xdr(e);

    let mut formatted_string_xdr = string_xdr.clone();

    for i in 0..string_xdr.len() {
        let ascii_val = string_xdr.get_unchecked(i);
        if ascii_val >= 97 && ascii_val <= 122 {
            formatted_string_xdr.set(i, ascii_val - 32);
        }
    }

    formatted_string_xdr
}

pub fn convert_to_lower(e: &Env, string: String) -> String {
    String::from_xdr(e, &to_lower_bytes(&e, string)).unwrap()
}

pub fn convert_to_upper(e: &Env, string: String) -> String {
    String::from_xdr(e, &to_upper_bytes(&e, string)).unwrap()
}
