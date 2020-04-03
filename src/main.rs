// This should be well-formed ASCII.
pub struct Ascii (
    // Inner `u8` needs between `0x00` and `0x7f`
    Vec<u8>
)

fn main() {
    println!("Hello, world!");
}
