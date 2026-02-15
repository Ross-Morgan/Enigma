pub trait BidirectionalPlug {
    fn plug_forwards(&mut self, c: char) -> char;
    fn plug_backwards(&mut self, c: char) -> char;
}

pub trait UnidirectionalPlug {
    fn plug(&self, c: char) -> char;
}
