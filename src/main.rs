// This should be well-formed ASCII.
#[derive(Debug, Eq, PartialEq)]
pub struct Ascii (
    // Inner `u8` needs between `0x00` and `0x7f`
    Vec<u8>
);

#[derive(Debug, Eq, PartialEq)]
pub struct NotAsciiError ();

impl Ascii {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|&byte| !byte.is_ascii()) {
            return Err(NotAsciiError())
        }
        Ok(Ascii(bytes))
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

    assert_eq!(not, NotAsciiError());
}

fn main() {
    println!("Hello, world!");
}
