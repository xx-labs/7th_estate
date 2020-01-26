//! Utility functions to deal with byte ordering.

pub mod le_bytes {
    pub fn from_slice_u16(slice: &[u16]) -> Box<[u8]> {
        let le_sequence: Vec<_> = slice.iter()
            .map(|&x| u16::to_le_bytes(x))
            .collect();
        let le_bytes: Vec<u8> = le_sequence.iter()
            .flat_map(|as_bytes| as_bytes.into_iter())
            .cloned()
            .collect();
        le_bytes.into_boxed_slice()
    }

    pub fn to_slice_u16(bytes: &[u8]) -> Box<[u16]> {
        let chunks = bytes.chunks_exact(std::mem::size_of::<u16>());
        assert_eq!(0, chunks.remainder().len());
        let ne_bytes: Vec<_> = chunks
            .map(|s| u16::from_le_bytes([s[0], s[1]]))
            .collect();
        ne_bytes.into_boxed_slice()
    }
}

