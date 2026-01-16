fn main() {
    println!("cargo:rerun-if-changed=./proto/auth.proto");
    println!("cargo:rerun-if-changed=./proto/statistics.proto");
    println!("cargo:rerun-if-changed=./proto/notification.proto");
    println!("cargo:rerun-if-changed=./proto/engagement.proto");
    println!("cargo:rerun-if-changed=./proto/content.proto");

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_protos(
            &[
                "./proto/auth.proto",
                "./proto/statistics.proto",
                "./proto/notification.proto",
                "./proto/engagement.proto",
                "./proto/content.proto",
            ],
            &["./proto"],
        )
        .expect("failed to compile protos");
}
