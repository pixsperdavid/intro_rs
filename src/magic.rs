pub trait Magic {
    fn is_magic(&self) -> bool;

    fn get_magic_value() -> Self;
}

impl Magic for u32 {
    fn is_magic(&self) -> bool {
        *self == Self::get_magic_value()
    }

    fn get_magic_value() -> Self {
        42
    }
}

impl Magic for String {
    fn is_magic(&self) -> bool {
        *self == Self::get_magic_value()
    }

    fn get_magic_value() -> Self {
        "abracadabra".to_string()
    }
}