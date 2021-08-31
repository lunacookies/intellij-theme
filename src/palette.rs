use std::ops::Range;
use tincture::{ColorSpace, Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 250.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.5454982, 0.16140655, 25.423063)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6524916, 0.13345489, 56.723343)
    }

    pub(crate) fn dark_orange(&self) -> Oklch {
        oklch(0.793238, 0.11389914, 75.47327)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.860106, 0.12477315, 76.95581)
    }

    pub(crate) fn tan(&self) -> Oklch {
        oklch(0.703477, 0.054352857, 83.247955)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.75477606, 0.15031359, 106.93401)
    }

    pub(crate) fn avocado(&self) -> Oklch {
        oklch(0.7766038, 0.08857281, 119.78649)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.58865094, 0.075113386, 134.45381)
    }

    pub(crate) fn bright_green(&self) -> Oklch {
        oklch(0.62251383, 0.11066254, 139.61948)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.62310046, 0.09924307, 198.42546)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.7159551, 0.06853732, 213.10477)
    }

    pub(crate) fn deep_blue(&self) -> Oklch {
        oklch(0.71396226, 0.12099062, 237.75323)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.65661603, 0.074155696, 242.23978)
    }

    pub(crate) fn bright_blue(&self) -> Oklch {
        oklch(0.5962281, 0.15484875, 260.67047)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.6176412, 0.085667424, 313.57822)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    DarkFg,
    Fg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.29..0.77)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.0..0.0266)
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::DarkFg => 0.65,
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
