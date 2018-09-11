error_chain! {
    foreign_links {
        Base64DecodeError(::base64::DecodeError);
        Io(::std::io::Error);
    }

    errors {
        ExpectedNullByte(found: u8) {
            description("expected a null byte"),
            display("expected a null byte, found: {:?}", found),
        }

        HeaderMismatch(expected: [u8; 2], actual: [u8; 2]) {
            description("input header did not match expected"),
            display("input header {:?} did not match expected {:?}", actual, expected),
        }

        InvalidKeyData(kd: String) {
            description("the given key data could not be parsed by <key>.<nonce> format"),
            display("invalid key data: {}", kd),
        }

        SourceNotFullyDrained(size: usize) {
            description("source vector was not fully drained"),
            display("source vector was not fully drained: {} elements remaining", size),
        }

        VersionMismatch(expected: u8, actual: u8) {
            description("tarbox header version mismatch"),
            display("tarbox header version mismatch: expected={} actual={}", expected, actual),
        }
    }
}
