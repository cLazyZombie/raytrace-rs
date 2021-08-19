use std::f32::consts::PI;

/// stores angle as radian
#[derive(Copy, Clone, Debug)]
pub struct Angle(f32);

impl Angle {
    pub fn from_radian(radian: f32) -> Self {
        Angle(radian)
    }

    pub fn from_degree(degree: f32) -> Self {
        let radian = degree.to_radians();
        Angle(radian)
    }

    pub fn radian(&self) -> f32 {
        self.0
    }

    pub fn degree(&self) -> f32 {
        self.0.to_degrees()
    }

    /// make radians in [0, 2*PI)
    pub fn normalize(self) -> Self {
        let mut radian = self.0 % (PI * 2.0);
        if radian < 0.0 {
            radian += PI * 2.0;
        }
        Self::from_radian(radian)
    }

    /// make radians in [-PI, PI)
    pub fn normalize_signed(self) -> Self {
        let mut radian = self.normalize().0;
        if radian >= PI * 2.0 {
            radian -= PI * 2.0;
        }
        Self::from_radian(radian)
    }
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
