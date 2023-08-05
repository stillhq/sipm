mod distrobox;
mod flatpak;
mod nativeifer;
mod snap;
mod wine;

pub const SOURCE_LIST: [String; 5] = [
    String::from("flatpak"),
    String::from("snap"),
    String::from("wine"),
    String::from("nativeifer"),
    String::from("distrobox")
];

pub enum SourceType {
    // Flatpak(flatpak::FlatpakSource),
    // Snap(snap::SnapSource),
    // Wine(wine::WineSource),
    // Nativeifer(nativeifer::NaviveierSource),
    Distrobox(distrobox::DistroboxSource)
}