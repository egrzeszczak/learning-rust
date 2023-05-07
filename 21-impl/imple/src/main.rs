struct Prostopadloscian {
    dlugosc: u32,
    wysokosc: u32,
    szerokosc: u32
}

impl Prostopadloscian {
    fn to_string(&self) {
        println!("Prostopadloscian {} x {} x {}", 
            self.dlugosc,
            self.wysokosc,
            self.szerokosc);
        println!("Czy to sześcian? {}", self.is_cube())
    }

    fn is_cube(&self) -> bool {
        return self.dlugosc == self.wysokosc && 
        self.wysokosc == self.szerokosc &&
        self.dlugosc == self.szerokosc;
    }
}

fn main() {
    let moj_prostopadloscian = Prostopadloscian {
        dlugosc: 12, wysokosc: 12, szerokosc: 12
    };

    moj_prostopadloscian.to_string();
}
