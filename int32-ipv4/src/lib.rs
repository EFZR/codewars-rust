// This code is converting an integer into an IP address format.
// The integer is divided into four parts, each representing one byte (8 bits) of the IP address.
// The '>>' operator is used to shift the bits of the integer to the right.
// For example, 'int >> 24' shifts the bits 24 places to the right, effectively isolating the leftmost byte.
// The '& 0xFF' operation is a bitwise AND with the hexadecimal number 0xFF (255 in decimal, 11111111 in binary).
// This operation masks the result of the shift, keeping only the last 8 bits.
// The 'format!' macro then combines these four bytes into a string, separated by dots.

pub fn int32_to_ip(int: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (int >> 24) & 0xFF,
        (int >> 16) & 0xFF,
        (int >> 8) & 0xFF,
        (int) & 0xFF
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
