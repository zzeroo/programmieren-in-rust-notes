pub enum KiType {
    thumb,
    smart,
}

pub struct Ki {
    ki_type: KiType,
}

impl Ki {
    pub fn new(ki_type: KiType) -> Self {
        Ki {
            ki_type: ki_type,
        }
    }
}
