use tinc_build;

fn main() {
    println!("cargo::rerun-if-changed=rms-protos");
    let mut cfg = tinc_build::Config::prost();

    cfg.disable_root_module();

    cfg.compile_protos(&["rms-protos/rms/rms_config.proto"], &["rms-protos"])
        .unwrap();
}
