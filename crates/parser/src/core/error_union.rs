#[macro_export]
macro_rules! combine_errors {
    ($vis:vis $name:ident, $($err:ident),+ $(,)?) => {
        use std::error::Error;
        use std::fmt::{self, Display, Formatter};

        #[derive(Debug)]
        $vis enum $name {
            $(
                $err($err),
            )+
        }

        $(
            impl From<$err> for $name {
                fn from(err: $err) -> Self {
                    $name::$err(err)
                }
            }
        )+

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        $name::$err(err) => write!(f, "{}", err),
                    )+
                }
            }
        }

        impl Error for $name {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match self {
                    $(
                        $name::$err(err) => Some(err),
                    )+
                }
            }
        }
    };
}
