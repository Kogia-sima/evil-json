fn main() {
    if version_check::is_feature_flaggable() == Some(true) {
        println!("cargo:rustc-cfg=evil_json_nightly");
    }

    if version_check::is_min_version("1.46.0") == Some(true) {
        println!("cargo:rustc-cfg=evil_json_1_46");
    }
}
