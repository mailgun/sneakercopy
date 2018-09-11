/// Shamelessly borrowed from https://jadpole.github.io/rust/builder-macro.
macro_rules! builder {
    ( $src_name:ident => $dest_name:ident { $($body:tt)* }) => {
        builder!{ () : $src_name => $dest_name { $($body)* } }
    };

    ( pub : $src_name:ident => $dest_name:ident { $($body:tt)* }) => {
        builder!{ (pub) : $src_name => $dest_name { $($body)* } }
    };

    ( ($($vis:tt)*) : $src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    }) => {
        #[derive(Clone, Debug)]
        $($vis)* struct $dest_name {
            $( $attr_name : $attr_type ),*
        }

        #[derive(Clone, Debug)]
        $($vis)* struct $src_name {
            $( $attr_name : Option<$attr_type> ),*
        }

        impl $src_name {
            pub fn new() -> $src_name {
                $src_name {
                    $(
                        $attr_name : $attr_default
                    ),*
                }
            }

            pub fn build(self) -> ::errors::Result<$dest_name> {
                $(
                    let $attr_name = self.$attr_name;
                    if $attr_name.is_none() {
                        bail!(
                            ::errors::ErrorKind::MissingField(
                                String::from(stringify!{ attr_name })
                            )
                        );
                    }

                    let $attr_name = $attr_name.unwrap();
                )*

                Ok($dest_name {
                    $( $attr_name : $attr_name ),*
                })
            }

            $(
                pub fn $attr_name(mut self, value: $attr_type) -> Self {
                    self.$attr_name = Some(value);
                    self
                }
            )*
        }
    };
}
