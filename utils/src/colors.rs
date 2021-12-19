pub enum Colors {
    Red,
    Blue,
    Green
}

impl Colors {
    pub fn to_color(&self) -> u32 {
        match self {
            Self::Red => 0xFF0000,
            Self::Blue => 0x0000FF,
            Self::Green => 0x00FF00
        }
    }
}