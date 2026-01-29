use tinc_build;

fn main() {
    println!("cargo::rerun-if-changed=rms-protos");
    let mut cfg = tinc_build::Config::prost();
    cfg.compile_protos(
        &[
            "rms-protos/rms/rms_config.proto",
            "rms-protos/rms/gsc_gateway.proto",
        ],
        &["rms-protos"],
    )
    .unwrap();
}
