#[macro_export]
macro_rules! matcher_gen {
    (enum $name:ident {
        $($opstring:literal => $variant:ident),*
    }) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $($variant,)*
        }
        impl $name {
            pub fn from_str(string: &str) -> Result<$name, std::io::Error> {
                match string {
                    $($opstring => Ok($name::$variant),)*
                    _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("'{}' equivalent variant not found in enum {}", string, stringify!($name))))
                }
            }
        }
    };
}
pub(crate) use matcher_gen;