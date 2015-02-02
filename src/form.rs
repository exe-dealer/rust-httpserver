use rustc_serialize::{
    Decoder,
    Decodable
};

pub type DecodeResult<T> = Result<T, DecodeError>;

type FormEntry = (String, String);

struct FormDecoder {
    form: Vec<FormEntry>,
    reading_values: Vec<String>,
}

impl FormDecoder {
    pub fn new(form: Vec<FormEntry>) -> FormDecoder {
        FormDecoder {
            form: form,
            reading_values: vec![]
        }
    }
}

#[derive(Show, PartialEq)]
pub enum DecodeError {
    MissingValue,
    TooManyValues,
    ParseError,
}

impl Decoder for FormDecoder {
    type Error = DecodeError;

    fn read_nil(&mut self) -> DecodeResult<()> { unimplemented!() }
    fn read_usize(&mut self) -> DecodeResult<usize> { unimplemented!() }
    fn read_u64(&mut self) -> DecodeResult<u64> { unimplemented!() }
    fn read_u32(&mut self) -> DecodeResult<u32> { unimplemented!() }
    fn read_u16(&mut self) -> DecodeResult<u16> { unimplemented!() }
    fn read_u8(&mut self) -> DecodeResult<u8> { unimplemented!() }
    fn read_isize(&mut self) -> DecodeResult<isize> { unimplemented!() }
    fn read_i64(&mut self) -> DecodeResult<i64> { unimplemented!() }
    fn read_i32(&mut self) -> DecodeResult<i32> { unimplemented!() }
    fn read_i16(&mut self) -> DecodeResult<i16> { unimplemented!() }
    fn read_i8(&mut self) -> DecodeResult<i8> { unimplemented!() }
    fn read_bool(&mut self) -> DecodeResult<bool> { unimplemented!() }
    fn read_f64(&mut self) -> DecodeResult<f64> { unimplemented!() }
    fn read_f32(&mut self) -> DecodeResult<f32> { unimplemented!() }
    fn read_char(&mut self) -> DecodeResult<char> { unimplemented!() }

    fn read_str(&mut self) -> DecodeResult<String> {
        self.reading_values.pop()
            .ok_or(DecodeError::MissingValue)
    }

    fn read_enum<T, F>(&mut self, name: &str, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodeResult<T> where F: FnMut(&mut Self, usize) -> DecodeResult<T> { unimplemented!() }
    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodeResult<T> where F: FnMut(&mut Self, usize) -> DecodeResult<T> { unimplemented!() }
    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> {
        // if (len != self.len) {
        //     panic!("Fields count mismatch.");
        // }
        f(self)
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> {
        let mut i = self.form.len();
        while (i > 0) {
            i -= 1;
            if self.form[i].0 == f_name {
                let (_, val) = self.form.swap_remove(i);
                self.reading_values.push(val);
            }
        }

        f(self)
    }

    fn read_tuple<T, F>(&mut self, len: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_option<T, F>(&mut self, mut f: F) -> DecodeResult<T> where F: FnMut(&mut Self, bool) -> DecodeResult<T> {
        let has_values = !self.reading_values.is_empty();
        f(self, has_values)
    }

    fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self, usize) -> DecodeResult<T> { unimplemented!() }
    fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_map<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self, usize) -> DecodeResult<T> { unimplemented!() }
    fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Self) -> DecodeResult<T> { unimplemented!() }
    fn error(&mut self, err: &str) -> DecodeError { unimplemented!() }
}


pub fn decode_form<T: Decodable>(form: Vec<(String, String)>) -> DecodeResult<T> {
    let mut decoder = FormDecoder::new(form);
    Decodable::decode(&mut decoder)
}





#[derive(Debug, PartialEq)]
pub enum PercentDecodeError {
    InvalidHexDigit(usize),
    TooShort,
}

fn percent_decode(input: &[u8]) -> Result<Vec<u8>, PercentDecodeError> {
    use self::PercentDecodeError::{ InvalidHexDigit, TooShort };

    let mut buf = Vec::with_capacity(input.len());
    let mut input_iter = input.iter().enumerate();

    fn read_hex(val: Option<(usize, &u8)>) -> Result<u8, PercentDecodeError> {
        val.ok_or(TooShort)
           .and_then(|(pos, &x)| from_hex(x).ok_or(InvalidHexDigit(pos)))
    }

    loop {
        match input_iter.next() {
            Some((_, &b'%')) => {
                let h = try!(read_hex(input_iter.next()));
                let l = try!(read_hex(input_iter.next()));
                buf.push(h * 0x10 + l);
            },
            Some((_, &b'+')) => buf.push(b' '),
            Some((_, &x)) => buf.push(x),
            None => break,
        }
    }

    Ok(buf)
}

fn from_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0' ... b'9' => Some(byte - b'0'),
        b'A' ... b'F' => Some(byte + 10 - b'A'),
        b'a' ... b'f' => Some(byte + 10 - b'a'),
        _ => None
    }
}




#[cfg(test)]
mod test {

    use super::{
        decode_form,
        DecodeError,
        percent_decode,
        PercentDecodeError,
    };



    #[test]
    fn decode_percent_str() {
        assert_eq!(
            percent_decode(b"%7Btest%7D+test"),
            Ok(b"{test} test".to_vec())
        );
    }

    #[test]
    fn decode_too_short_percent() {
        assert_eq!(
            percent_decode(b"abc%0"),
            Err(PercentDecodeError::TooShort)
        );
    }

    #[test]
    fn decode_invalid_hex_percent() {
        assert_eq!(
            percent_decode(b"abc%G"),
            Err(PercentDecodeError::InvalidHexDigit(4))
        );
    }



    #[test]
    fn decode_two_strings() {

        #[derive(RustcDecodable, Show, PartialEq)]
        struct FooBar {
            database: String,
            sql_script: String,
        }

        let result = decode_form::<FooBar>(vec![
            ("database".to_string(), "postgres".to_string()),
            ("sql_script".to_string(), "select 'hello'".to_string()),
        ]);

        assert_eq!(result, Ok(FooBar {
            database: "postgres".to_string(),
            sql_script: "select 'hello'".to_string(),
        }));
    }
}


