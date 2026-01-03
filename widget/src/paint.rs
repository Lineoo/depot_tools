use cosmic_text::Color as CosmicColor;
use sdl3::pixels::Color as SdlColor;

pub mod creator;
pub mod painter;
pub mod shapes;

pub fn sdlColorIntoCosmicColor(color: SdlColor) -> CosmicColor {
    CosmicColor::rgba(color.r, color.g, color.b, color.a)
}

pub fn cosmicColorIntoSdlColor(color: CosmicColor) -> SdlColor {
    SdlColor {
        r: color.r(),
        g: color.g(),
        b: color.b(),
        a: color.a(),
    }
}
