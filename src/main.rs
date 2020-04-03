use std::fmt;

// This should be well-formed ASCII.
#[derive(Debug, Eq, PartialEq)]
pub struct Ascii (
    // Inner `u8` needs between `0x00` and `0x7f`
    Vec<u8>
);

#[derive(Debug, Eq, PartialEq)]
pub struct NotAsciiError (&'static str);

impl fmt::Display for NotAsciiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{}", self.0)
    }
}

impl Ascii {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|&byte| !byte.is_ascii()) {
            return Err(NotAsciiError("not well-formed"))
        }
        Ok(Ascii(bytes))
    }
}

impl From<Ascii> for String {
    fn from(ascii: Ascii) -> Self {
        // my ascii is well-formed, checked.
        unsafe { String::from_utf8_unchecked(ascii.0) }
    }
}

#[test]
fn well_ascii() {
    let well_formed_ascii = vec![0x00, 0x41, 0x7f];
    let well = Ascii::from_bytes(well_formed_ascii).ok().unwrap();

    assert_eq!(well,
        Ascii(vec!['\x00' as u8, '\x41' as u8, '\x7f' as u8]));
}

#[test]
fn not_ascii() {
    let not_ascii = vec![0x00, 0x80, 0xff];
    let not = Ascii::from_bytes(not_ascii).err().unwrap();

    assert_eq!(not, NotAsciiError("not well-formed"));
    println!("{:}", not);
}

#[test]
fn my_ascii_to_string() {
    let my_ascii = Ascii::from_bytes(
        vec!['\x68' as u8, '\x65' as u8,
            '\x6C' as u8, '\x6C' as u8, '\x6F' as u8]
        )
        .ok().unwrap();

    assert_eq!(String::from(my_ascii), "hello".to_string());
}

fn main() {
    println!("Hello, world!");
}
