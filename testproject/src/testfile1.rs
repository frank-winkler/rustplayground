mod test;

pub struct Modifier {
    inludes: (i8, i8)
}

impl Modifier {
    pub fn modify(&mut self) {
        self.includes = (1, 8)
    }
}
