//! OpenType unsigned-word checksums.

use crate::Tag;

/// The wrapping sum of big-endian `u32` words in a table.
///
/// ```
/// use glyphweave_sfnt::Checksum;
///
/// let checksum = Checksum::compute(&[1, 2, 3, 4, 5]);
/// assert_eq!(checksum.raw(), 0x0602_0304);
/// ```
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
        Self::compute_inner(bytes, false)
    }

    /// Computes the checksum declared for `tag` in a Table Record.
    ///
    /// For `head`, bytes 8 through 11 are treated as zero because they hold the whole-font
    /// checksum adjustment written after the table checksum is recorded.
    #[must_use]
    pub fn compute_for(tag: Tag, bytes: &[u8]) -> Self {
        Self::compute_inner(bytes, tag == Tag::new(*b"head"))
    }

    fn compute_inner(bytes: &[u8], zero_adjustment: bool) -> Self {
        let mut sum = 0_u32;
        for (word_index, chunk) in bytes.chunks(4).enumerate() {
            let mut word = [0_u8; 4];
            word[..chunk.len()].copy_from_slice(chunk);
            if zero_adjustment && word_index == 2 {
                word = [0; 4];
            }
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

    #[test]
    fn head_checksum_ignores_checksum_adjustment_word() {
        let bytes = [0, 0, 0, 1, 0, 0, 0, 2, 0xde, 0xad, 0xbe, 0xef];
        assert_eq!(Checksum::compute(&bytes).raw(), 0xdead_bef2);
        assert_eq!(
            Checksum::compute_for(crate::Tag::new(*b"head"), &bytes).raw(),
            3
        );
        assert_eq!(
            Checksum::compute_for(crate::Tag::new(*b"name"), &bytes),
            Checksum::compute(&bytes)
        );
    }
}
