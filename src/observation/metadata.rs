use std::ops::{BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy)]
pub struct ObservationMetadata {
    pub changed: bool,
    pub new: bool,
}

impl Default for ObservationMetadata {
    fn default() -> Self {
        ObservationMetadata {
            changed: false,
            new: false,
        }
    }
}

impl BitOr for ObservationMetadata {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        ObservationMetadata {
            changed: self.changed || rhs.changed,
            new: self.new || rhs.new,
        }
    }
}

impl BitOrAssign for ObservationMetadata {
    fn bitor_assign(&mut self, rhs: Self) {
        self.changed = self.changed || rhs.changed;
        self.new = self.new || rhs.new;
    }
}
