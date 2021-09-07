use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) enum Palette {
    Normal,
    GreyscaleBase,
}

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let chroma = match self {
            Self::Normal => scale.chroma(),
            Self::GreyscaleBase => 0.0,
        };

        oklch(scale.lightness(), chroma, 250.0)
    }

    pub(crate) fn ui_blue(&self) -> Oklch {
        oklch(0.61989343, 0.117343895, 250.59079)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.5454982, 0.16140655, 25.423063)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6524916, 0.13345489, 56.723343)
    }

    pub(crate) fn beige(&self) -> Oklch {
        oklch(0.7767229, 0.029570978, 65.62064)
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

    pub(crate) fn sea_green(&self) -> Oklch {
        oklch(0.67939675, 0.037723456, 169.02066)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.62310046, 0.09924307, 198.42546)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.7159551, 0.06853732, 213.10477)
    }

    pub(crate) fn dark_cyan(&self) -> Oklch {
        oklch(0.66326016, 0.042912994, 218.54863)
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

    pub(crate) fn pale_lavender(&self) -> Oklch {
        oklch(0.79945934, 0.029780919, 279.20978)
    }

    pub(crate) fn lavender(&self) -> Oklch {
        oklch(0.79074275, 0.06365498, 283.82767)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.6176412, 0.085667424, 313.57822)
    }

    pub(crate) fn ansi_red(&self) -> Oklch {
        oklch(0.65427667, 0.19467114, 25.222906)
    }

    pub(crate) fn ansi_green(&self) -> Oklch {
        oklch(0.6112721, 0.14990547, 133.87198)
    }

    pub(crate) fn ansi_yellow(&self) -> Oklch {
        oklch(0.6407205, 0.12850519, 94.13027)
    }

    pub(crate) fn ansi_blue(&self) -> Oklch {
        oklch(0.63923806, 0.12909275, 243.93295)
    }

    pub(crate) fn ansi_magenta(&self) -> Oklch {
        oklch(0.6338902, 0.12722933, 314.88364)
    }

    pub(crate) fn ansi_cyan(&self) -> Oklch {
        oklch(0.6477781, 0.110605896, 194.79843)
    }

    pub(crate) fn ansi_bright_red(&self) -> Oklch {
        oklch(0.6619153, 0.22514442, 21.791573)
    }

    pub(crate) fn ansi_bright_green(&self) -> Oklch {
        oklch(0.72433686, 0.22107385, 138.65419)
    }

    pub(crate) fn ansi_bright_yellow(&self) -> Oklch {
        oklch(0.8137701, 0.1668647, 94.43816)
    }

    pub(crate) fn ansi_bright_blue(&self) -> Oklch {
        oklch(0.72313845, 0.1600398, 240.48105)
    }

    pub(crate) fn ansi_bright_magenta(&self) -> Oklch {
        oklch(0.75472784, 0.19105218, 327.246)
    }

    pub(crate) fn ansi_bright_cyan(&self) -> Oklch {
        oklch(0.8346787, 0.1425187, 194.79842)
    }

    pub(crate) fn type_icon(&self) -> Oklch {
        oklch(0.7282886, 0.11905296, 225.98761)
    }

    pub(crate) fn function_icon(&self) -> Oklch {
        oklch(0.7602619, 0.1342758, 9.718126)
    }

    pub(crate) fn interface_icon(&self) -> Oklch {
        oklch(0.69615144, 0.17064784, 138.26198)
    }

    pub(crate) fn property_icon(&self) -> Oklch {
        oklch(0.7519401, 0.13388576, 297.94778)
    }

    pub(crate) fn variable_icon(&self) -> Oklch {
        oklch(0.8005755, 0.14743003, 75.65576)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    LightBg,
    LighterBg,
    BrightBg,
    BrighterBg,
    BrightestBg,
    DarkerFg,
    DarkFg,
    FadedFg,
    Fg,
    LightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.29..0.77)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::Bg | Self::LightBg | Self::LightFg => 0.0,
            _ => lerp(self.value(), 0.0..0.0266),
        }
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::LightBg => 0.05,
            Self::LighterBg => 0.06,
            Self::BrightBg => 0.15,
            Self::BrighterBg => 0.23,
            Self::BrightestBg => 0.3,
            Self::DarkerFg => 0.45,
            Self::DarkFg => 0.65,
            Self::FadedFg => 0.83,
            Self::Fg => 1.0,
            Self::LightFg => 1.05,
        }
    }
}

pub(crate) fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
