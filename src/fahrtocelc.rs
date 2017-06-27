pub struct FahrToCelc {
    fahr: f32,
    step: f32,
}

impl FahrToCelc {
    pub fn new(fahr: f32, step: f32) -> FahrToCelc {
        FahrToCelc {
            fahr: fahr,
            step: step,
        }
    }
}

impl Iterator for FahrToCelc {
    type Item = (f32, f32);

    fn next (&mut self) -> Option<Self::Item> {
        let curr_fahr = self.fahr;
        let curr_celc = (self.fahr - 32.0) / 1.8;
        self.fahr = self.fahr + self.step;
        Some((curr_fahr, curr_celc))
    }
}
