//! OpenType unsigned-word checksums.

/// The wrapping sum of big-endian `u32` words in a table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Checksum(u32);

impl Checksum {
    /// Preserves a checksum value read from a Table Record.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Computes a checksum, treating a final partial word as zero-padded.
    #[must_use]
    pub fn compute(bytes: &[u8]) -> Self {
        let mut sum = 0_u32;
        for chunk in bytes.chunks(4) {
            let mut word = [0_u8; 4];
            word[..chunk.len()].copy_from_slice(chunk);
            sum = sum.wrapping_add(u32::from_be_bytes(word));
        }
        Self(sum)
    }

    /// The encoded unsigned value.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Checksum;

    #[test]
    fn zero_pads_the_final_partial_word() {
        let checksum = Checksum::compute(&[1, 2, 3, 4, 5]);
        assert_eq!(checksum.raw(), 0x0102_0304 + 0x0500_0000);
    }

    #[test]
    fn addition_wraps_at_thirty_two_bits() {
        let checksum = Checksum::compute(&[0xff; 8]);
        assert_eq!(checksum.raw(), 0xffff_fffe);
    }

    #[test]
    fn empty_table_has_zero_checksum() {
        assert_eq!(Checksum::compute(&[]), Checksum::default());
    }
}
