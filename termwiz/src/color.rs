//! Colors for attributes
// for FromPrimitive
#![allow(clippy::useless_attribute)]

#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};
pub use wezterm_color_types::{LinearRgba, SrgbaTuple};
use wezterm_dynamic::{FromDynamic, ToDynamic};

pub use wezterm_escape_parser::color::{AnsiColor, ColorSpec, PaletteIndex, RgbColor};

/// Specifies the color to be used when rendering a cell.  This is the
/// type used in the `CellAttributes` struct and can specify an optional
/// TrueColor value, allowing a fallback to a more traditional palette
/// index if TrueColor is not available.
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromDynamic, ToDynamic, Hash)]
pub enum ColorAttribute {
    /// Use RgbColor when supported, falling back to the specified PaletteIndex.
    TrueColorWithPaletteFallback(SrgbaTuple, PaletteIndex),
    /// Use RgbColor when supported, falling back to the default color
    TrueColorWithDefaultFallback(SrgbaTuple),
    /// Use the specified PaletteIndex
    PaletteIndex(PaletteIndex),
    /// Use the default color
    Default,
}

impl Default for ColorAttribute {
    fn default() -> Self {
        ColorAttribute::Default
    }
}

impl From<AnsiColor> for ColorAttribute {
    fn from(col: AnsiColor) -> Self {
        ColorAttribute::PaletteIndex(col as u8)
    }
}

impl From<ColorSpec> for ColorAttribute {
    fn from(spec: ColorSpec) -> Self {
        match spec {
            ColorSpec::Default => ColorAttribute::Default,
            ColorSpec::PaletteIndex(idx) => ColorAttribute::PaletteIndex(idx),
            ColorSpec::TrueColor(color) => ColorAttribute::TrueColorWithDefaultFallback(color),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_hsl() {
        let foo = RgbColor::from_rgb_str("hsl:235 100  50").unwrap();
        assert_eq!(foo.to_rgb_string(), "#0015ff");
    }

    #[test]
    fn from_rgb() {
        assert!(RgbColor::from_rgb_str("").is_none());
        assert!(RgbColor::from_rgb_str("#xyxyxy").is_none());

        let black = RgbColor::from_rgb_str("#FFF").unwrap();
        assert_eq!(black.to_tuple_rgb8(), (0xf0, 0xf0, 0xf0));

        let black = RgbColor::from_rgb_str("#000000").unwrap();
        assert_eq!(black.to_tuple_rgb8(), (0, 0, 0));

        let grey = RgbColor::from_rgb_str("rgb:D6/D6/D6").unwrap();
        assert_eq!(grey.to_tuple_rgb8(), (0xd6, 0xd6, 0xd6));

        let grey = RgbColor::from_rgb_str("rgb:f0f0/f0f0/f0f0").unwrap();
        assert_eq!(grey.to_tuple_rgb8(), (0xf0, 0xf0, 0xf0));
    }

    #[cfg(feature = "use_serde")]
    #[test]
    fn roundtrip_rgbcolor() {
        let data = varbincode::serialize(&RgbColor::from_named("DarkGreen").unwrap()).unwrap();
        eprintln!("serialized as {:?}", data);
        let _decoded: RgbColor = varbincode::deserialize(data.as_slice()).unwrap();
    }
}
