use std::ops::Range;
use tincture::{ColorSpace, Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 250.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.6524916, 0.13345489, 56.723343)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.860106, 0.12477315, 76.95581)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.58865094, 0.075113386, 134.45381)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.65661603, 0.074155696, 242.23978)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.6176412, 0.085667424, 313.57822)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    Fg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.29..0.77)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::Fg => 0.0266,
        }
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::Fg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
