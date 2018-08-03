error_chain! {
    foreign_links {
        Base64DecodeError(::base64::DecodeError);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }

    errors {
        InvalidKeyData(kd: String) {
            description("the given key data could not be parsed by <key>.<nonce> format"),
            display("invalid key data: {}", kd),
        }

        PathDoesNotExist(path: String) {
            description("the file or directory specified does not exist"),
            display("file or directory does not exist: {}", path),
        }

        SecretBoxOpenFail {
            description("could not open secretbox"),
            display("could not open secretbox"),
        }
    }
}