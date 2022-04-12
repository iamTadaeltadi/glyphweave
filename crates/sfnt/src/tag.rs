//! The Tag primitive: a four-byte OpenType identifier.

use core::fmt::{self, Write as _};

/// A four-byte identifier such as `cmap`, `glyf`, or `GSUB`.
///
/// Tags identify tables in a Table Directory, scripts and features in a Layout Table, and the
/// version of a Face. They compare and order as raw big-endian bytes, which is exactly the
/// ordering the OpenType Table Directory requires.
///
/// ```
/// use glyphweave_sfnt::Tag;
///
/// let cmap = Tag::new(*b"cmap");
/// assert_eq!(cmap.to_string(), "cmap");
/// assert_eq!(cmap, Tag::from_u32(0x636d_6170));
/// assert!(Tag::new(*b"GPOS") < Tag::new(*b"GSUB"));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag([u8; 4]);

impl Tag {
    /// Creates a tag from four raw bytes.
    #[must_use]
    pub const fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    /// Creates a tag from its big-endian `u32` encoding.
    #[must_use]
    pub const fn from_u32(value: u32) -> Self {
        Self(value.to_be_bytes())
    }

    /// The four raw bytes of the tag.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 4] {
        self.0
    }

    /// The big-endian `u32` encoding of the tag.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        u32::from_be_bytes(self.0)
    }

    /// Returns `true` when the bytes satisfy the OpenType Tag encoding rules.
    ///
    /// A valid Tag contains one to four printable ASCII non-space bytes followed only by
    /// optional trailing spaces.
    #[must_use]
    pub fn is_well_formed(self) -> bool {
        let mut saw_space = false;
        let mut non_space = 0;
        for byte in self.0 {
            if !(0x20..=0x7e).contains(&byte) {
                return false;
            }
            if byte == b' ' {
                saw_space = true;
            } else if saw_space {
                return false;
            } else {
                non_space += 1;
            }
        }
        non_space > 0
    }
}

impl fmt::Display for Tag {
    /// Renders printable ASCII bytes verbatim and escapes every other byte as `\xNN`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in &self.0 {
            if byte == b' ' || byte.is_ascii_graphic() {
                f.write_char(char::from(byte))?;
            } else {
                write!(f, "\\x{byte:02x}")?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tag({self})")
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn bytes_round_trip_through_u32() {
        let tag = Tag::new(*b"glyf");
        assert_eq!(tag.to_u32(), 0x676c_7966);
        assert_eq!(Tag::from_u32(tag.to_u32()), tag);
        assert_eq!(tag.to_bytes(), *b"glyf");
    }

    #[test]
    fn ordering_matches_big_endian_bytes() {
        assert!(Tag::new(*b"GPOS") < Tag::new(*b"GSUB"));
        assert!(Tag::new(*b"cmap") < Tag::new(*b"head"));
        assert_eq!(Tag::new(*b"OS/2").to_u32(), 0x4f53_2f32);
    }

    #[test]
    fn display_renders_ascii_tags() {
        assert_eq!(Tag::new(*b"cmap").to_string(), "cmap");
        assert_eq!(Tag::new(*b"OS/2").to_string(), "OS/2");
        assert_eq!(Tag::new([b'v', b'e', b'r', b' ']).to_string(), "ver ");
    }

    #[test]
    fn display_escapes_non_printable_bytes() {
        assert_eq!(
            Tag::new([0x00, 0x01, b'A', 0x7f]).to_string(),
            "\\x00\\x01A\\x7f"
        );
    }

    #[test]
    fn debug_wraps_the_rendered_tag() {
        assert_eq!(format!("{:?}", Tag::new(*b"name")), "Tag(name)");
    }

    #[test]
    fn well_formed_tags_are_printable_and_trailing_space_padded() {
        assert!(Tag::new(*b"cmap").is_well_formed());
        assert!(Tag::new(*b"CFF ").is_well_formed());
        assert!(!Tag::new(*b"    ").is_well_formed());
        assert!(!Tag::new(*b"a b ").is_well_formed());
        assert!(!Tag::new([b'a', b'b', b'c', 0x7f]).is_well_formed());
    }
}
