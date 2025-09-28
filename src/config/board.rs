#[derive(Debug, Clone)]
pub enum Board {
    Barisal,
    Chittagong,
    Comilla,
    Dhaka,
    Dinajpur,
    Jessore,
    Mymensingh,
    Rajshahi,
    Sylhet,
    Madrasah,
    Technical,
    Dibs,
}

impl Board {
    pub fn as_str(&self) -> &'static str {
        match self {
            Board::Barisal => "barisal",
            Board::Chittagong => "chittagong",
            Board::Comilla => "comilla",
            Board::Dhaka => "dhaka",
            Board::Dinajpur => "dinajpur",
            Board::Jessore => "jessore",
            Board::Mymensingh => "mymensingh",
            Board::Rajshahi => "rajshahi",
            Board::Sylhet => "sylhet",
            Board::Madrasah => "madrasah",
            Board::Technical => "tec",
            Board::Dibs => "dibs",
        }
    }
}
