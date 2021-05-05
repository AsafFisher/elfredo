pub const EMBEDDED_MAGIC: &[u8; 4] = b"\xDE\xAD\xBE\xEF";
pub struct DataEntry{
    magic: [u8; 4],
    size: usize,
    data: [u8],
}