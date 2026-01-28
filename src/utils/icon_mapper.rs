use slint::{Image, SharedPixelBuffer, Rgba8Pixel};
#[allow(unused_imports)]
use std::path::PathBuf;

pub fn get_initial(name: &str) -> String {
    name.chars().next().unwrap_or('?').to_uppercase().to_string()
}

pub fn map_name_to_icon_key(name: &str) -> Option<&'static str> {
    let lower_name = name.to_lowercase();
    if lower_name.contains("ubuntu") { Some("ubuntu") }
    else if lower_name.contains("debian") { Some("debian") }
    else if lower_name.contains("kali") { Some("kali") }
    else if lower_name.contains("fedora") || lower_name.contains("fed") { Some("fedora") }
    else if lower_name.contains("opensuse") || lower_name.contains("suse") { Some("opensuse") }
    else if lower_name.contains("arch") { Some("arch") }
    else if lower_name.contains("mint") { Some("mint") }
    else if lower_name.contains("alpine") { Some("alpine") }
    else if lower_name.contains("manjaro") { Some("manjaro") }
    else if lower_name.contains("elementary") { Some("elementary") }
    else if lower_name.contains("pop") { Some("pop_os") }
    else if lower_name.contains("centos") { Some("centos") }
    else if lower_name.contains("alma") { Some("almalinux") }
    else if lower_name.contains("rocky") { Some("rockylinux") }
    else if lower_name.contains("oracle") || lower_name == "ol" { Some("oracle") }
    else if lower_name.contains("gentoo") { Some("gentoo") }
    else if lower_name.contains("freebsd") { Some("freebsd") }
    else if lower_name.contains("zorin") { Some("zorin") }
    else if lower_name.contains("nix") { Some("nixos") }
    else if lower_name.contains("puppy") { Some("puppy") }
    else if lower_name.contains("penguin") { Some("penguin") }
    else if lower_name.contains("cachy") { Some("cachyos") }
    else if lower_name.contains("mx") { Some("mxlinux") }
    else if lower_name.contains("endeavour") { Some("endeavouros") }
    else if lower_name.contains("nobara") { Some("nobara") }
    else if lower_name.contains("anduin") { Some("anduinos") }
    else if lower_name.contains("neon") { Some("kdeneon") }
    else if lower_name.contains("bluestar") { Some("bluestar") }
    else if lower_name.contains("antix") { Some("antix") }
    else if lower_name.contains("tuxedo") { Some("tuxedo") }
    else if lower_name.contains("garuda") { Some("garuda") }
    else if lower_name.contains("biglinux") { Some("biglinux") }
    else if lower_name.contains("q4os") { Some("q4os") }
    else if lower_name.contains("sparky") { Some("sparky") }
    else if lower_name.contains("solus") { Some("solus") }
    else { None }
}

pub fn get_display_name(key: Option<&str>) -> String {
    match key {
        Some("ubuntu") => "Ubuntu".to_string(),
        Some("debian") => "Debian".to_string(),
        Some("kali") => "Kali Linux".to_string(),
        Some("fedora") => "Fedora".to_string(),
        Some("opensuse") => "openSUSE".to_string(),
        Some("arch") => "Arch Linux".to_string(),
        Some("mint") => "Linux Mint".to_string(),
        Some("alpine") => "Alpine Linux".to_string(),
        Some("manjaro") => "Manjaro".to_string(),
        Some("elementary") => "elementary OS".to_string(),
        Some("pop_os") => "Pop!_OS".to_string(),
        Some("centos") => "CentOS".to_string(),
        Some("almalinux") => "AlmaLinux".to_string(),
        Some("rockylinux") => "Rocky Linux".to_string(),
        Some("oracle") => "Oracle Linux".to_string(),
        Some("gentoo") => "Gentoo".to_string(),
        Some("freebsd") => "FreeBSD".to_string(),
        Some("zorin") => "Zorin OS".to_string(),
        Some("nixos") => "NixOS".to_string(),
        Some("puppy") => "Puppy Linux".to_string(),
        Some("penguin") => "Linux".to_string(),
        Some("cachyos") => "CachyOS".to_string(),
        Some("mxlinux") => "MX Linux".to_string(),
        Some("endeavouros") => "EndeavourOS".to_string(),
        Some("nobara") => "Nobara".to_string(),
        Some("anduinos") => "AnduinOS".to_string(),
        Some("kdeneon") => "KDE neon".to_string(),
        Some("bluestar") => "Bluestar Linux".to_string(),
        Some("antix") => "antiX".to_string(),
        Some("tuxedo") => "TUXEDO OS".to_string(),
        Some("garuda") => "Garuda Linux".to_string(),
        Some("biglinux") => "BigLinux".to_string(),
        Some("q4os") => "Q4OS".to_string(),
        Some("sparky") => "SparkyLinux".to_string(),
        Some("solus") => "Solus".to_string(),
        _ => "".to_string(),
    }
}

fn load_png(data: &[u8]) -> Image {
    let img = image::load_from_memory(data).expect("Failed to load PNG from memory");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    buffer.make_mut_bytes().copy_from_slice(rgba.as_raw());
    Image::from_rgba8(buffer)
}

fn load_svg(data: &[u8]) -> Image {
    Image::load_from_svg_data(data).expect("Failed to load SVG from memory")
}

pub fn load_icon(key: &str) -> Option<Image> {
    match key {
        "almalinux" => Some(load_png(include_bytes!("../../assets/icons/almalinux.png"))),
        "alpine" => Some(load_png(include_bytes!("../../assets/icons/alpine.png"))),
        "anduinos" => Some(load_png(include_bytes!("../../assets/icons/anduinos.png"))),
        "antix" => Some(load_png(include_bytes!("../../assets/icons/antix.png"))),
        "arch" => Some(load_png(include_bytes!("../../assets/icons/arch.png"))),
        "centos" => Some(load_svg(include_bytes!("../../assets/icons/centos.svg"))),
        "debian" => Some(load_svg(include_bytes!("../../assets/icons/debian.svg"))),
        "elementary" => Some(load_png(include_bytes!("../../assets/icons/elementary.png"))),
        "endeavouros" => Some(load_png(include_bytes!("../../assets/icons/endeavouros.png"))),
        "fedora" => Some(load_png(include_bytes!("../../assets/icons/fedora.png"))),
        "freebsd" => Some(load_png(include_bytes!("../../assets/icons/freebsd.png"))),
        "garuda" => Some(load_png(include_bytes!("../../assets/icons/garuda.png"))),
        "gentoo" => Some(load_png(include_bytes!("../../assets/icons/gentoo.png"))),
        "kali" => Some(load_png(include_bytes!("../../assets/icons/kali.png"))),
        "kdeneon" => Some(load_png(include_bytes!("../../assets/icons/kdeneon.png"))),
        "manjaro" => Some(load_png(include_bytes!("../../assets/icons/manjaro.png"))),
        "mint" => Some(load_png(include_bytes!("../../assets/icons/mint.png"))),
        "mxlinux" => Some(load_png(include_bytes!("../../assets/icons/mxlinux.png"))),
        "nixos" => Some(load_png(include_bytes!("../../assets/icons/nixos.png"))),
        "nobara" => Some(load_png(include_bytes!("../../assets/icons/nobara.png"))),
        "opensuse" => Some(load_png(include_bytes!("../../assets/icons/opensuse.png"))),
        "oracle" => Some(load_png(include_bytes!("../../assets/icons/oracle.png"))),
        "pop_os" => Some(load_png(include_bytes!("../../assets/icons/pop_os.png"))),
        "puppy" => Some(load_png(include_bytes!("../../assets/icons/puppy.png"))),
        "rockylinux" => Some(load_svg(include_bytes!("../../assets/icons/rockylinux.svg"))),
        "solus" => Some(load_png(include_bytes!("../../assets/icons/solus.png"))),
        "tuxedo" => Some(load_png(include_bytes!("../../assets/icons/tuxedo.png"))),
        "ubuntu" => Some(load_png(include_bytes!("../../assets/icons/ubuntu.png"))),
        "zorin" => Some(load_png(include_bytes!("../../assets/icons/zorin.png"))),
        _ => None,
    }
}
