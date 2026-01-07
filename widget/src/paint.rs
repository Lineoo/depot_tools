use cosmic_text::Color as CosmicColor;
use sdl3::pixels::Color as SdlColor;

pub mod creator;
pub mod painter;
pub mod shapes;

pub fn sdl_color_into_cosmic_color(color: SdlColor) -> CosmicColor {
    CosmicColor::rgba(color.r, color.g, color.b, color.a)
}

pub fn cosmic_color_into_sdl_color(color: CosmicColor) -> SdlColor {
    SdlColor {
        r: color.r(),
        g: color.g(),
        b: color.b(),
        a: color.a(),
    }
}
