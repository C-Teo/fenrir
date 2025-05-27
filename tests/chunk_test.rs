#[cfg(test)]
mod tests {
    use fenrir::chunk::Chunk;
    use fenrir::value::Value;

    #[test]
    fn write_chunk_single_byte() {
        let mut chunk = Chunk::new();
        chunk.write_chunk(0x01, 1);

        assert_eq!(chunk.code.len(), 1, "Code vector should contain one byte.");
        assert_eq!(chunk.code[0], 0x01, "Unexpected byte in the code vector.");
        assert_eq!(
            chunk.lines,
            vec![(1, 1)],
            "Lines vector should match the line and count."
        );
    }
}