#[macro_export]
macro_rules! matcher_gen {
    (enum $name:ident {
        $($opstring:literal => $instruction:ident),*
    }) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $($instruction,)*
        }
        impl $name {
            pub fn from_str(string: &str) -> Result<$name, std::io::Error> {
                match string {
                    $($opstring => Ok($name::$instruction),)*
                    _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{} instruction not found !", string)))
                }
            }
        }
    };
}
pub(crate) use matcher_gen;