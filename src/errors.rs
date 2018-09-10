error_chain! {
    links {
        Tarbox(::tarbox::errors::Error, ::tarbox::errors::ErrorKind);
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }

    errors {
        MissingField(name: String) {
            description("field missing during build"),
            display("field missing during build: {}", name),
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
