use crate::utils::is_hexa;


pub fn is_crc32(crc32 : &str) -> bool {
    is_hexa(crc32) && crc32.len() == 8
}