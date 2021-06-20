//! # Art
//!
//! A library for modeling artistic concepts.

// The pub use statements below allow us to re-export functions
// and enums from the modules below at the top-level. This allows
// consumers of the crate to access these values as use art::PrimaryColor,
// use art::SecondaryColor, and use art::mix, respectively.
// The main goal here is to decouple the internal API from the public API.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
