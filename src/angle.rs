#[derive(Copy, Clone, Debug)]
pub enum Angle {
    Radian(f32),
    Degree(f32),
}

impl Angle {
    pub fn from_radian(radian: f32) -> Self {
        Angle::Radian(radian)
    }

    pub fn from_degree(degree: f32) -> Self {
        Angle::Degree(degree)
    }

    pub fn radian(&self) -> f32 {
        match self {
            &Angle::Degree(degree) => f32::to_radians(degree),
            &Angle::Radian(radian) => radian,
        }
    }

    pub fn degree(&self) -> f32 {
        match self {
            &Angle::Degree(degree) => degree,
            &Angle::Radian(radian) => f32::to_degrees(radian),
        }
    }

    // pub fn almost_eq(&self, other: Angle) -> bool {
    // 	let rad1 = self.radian();
    // 	let rad2 = other.radian();
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_radian() {
        let r = Angle::from_radian(1.0);
        assert_eq!(r.radian(), 1.0);
    }
}
