fn main() {
    // tauri-build embeds `icons/icon.ico` as the Windows executable resource, which
    // is also where windows and the tray take their icon from, but it only asks to
    // rerun when the configuration changes. Track the icons too, so regenerating
    // them never leaves a stale icon in the binary.
    println!("cargo:rerun-if-changed=icons");
    tauri_build::build()
}
