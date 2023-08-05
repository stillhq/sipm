use super::*;

mod distrobox;
mod flatpak;
mod nativeifer;
mod snap;
mod wine;

pub const SOURCE_LIST: &[&str] = &["flatpak", "snap", "wine", "nativeifer", "distrobox"];

pub enum SourceType {
    // Flatpak(flatpak::FlatpakSource),
    // Snap(snap::SnapSource),
    // Wine(wine::WineSource),
    // Nativeifer(nativeifer::NaviveierSource),
    Distrobox(distrobox::DistroboxSource),
}

