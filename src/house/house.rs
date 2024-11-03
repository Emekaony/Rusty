struct House {
    number_of_rooms: i8,
    number_of_floors: i8,
}

impl House {
    pub fn is_duplex(&self) -> bool {
        return self.number_of_floors > 1;
    }
    pub fn can_contain_legion(&self) -> bool {
        return self.number_of_rooms > 10;
    }
}
