use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use rust_windows_spotify_widget::icon::generate_icon_rgba;
use std::fs::File;

/// Standard Windows icon sizes — Explorer, taskbar, and alt-tab each pick
/// whichever of these fits best for their context.
const SIZES: [u32; 6] = [16, 32, 48, 64, 128, 256];

fn main() {
    let mut icon_dir = IconDir::new(ResourceType::Icon);

    for size in SIZES {
        let rgba = generate_icon_rgba(size);
        let image = IconImage::from_rgba_data(size, size, rgba);
        let entry = IconDirEntry::encode(&image).expect("failed to encode icon frame");
        icon_dir.add_entry(entry);
    }

    std::fs::create_dir_all("assets").expect("failed to create assets/ directory");
    let file = File::create("assets/icon.ico").expect("failed to create assets/icon.ico");
    icon_dir.write(file).expect("failed to write icon.ico");

    println!("Wrote assets/icon.ico with sizes: {:?}", SIZES);
}