use std::io;

/// I've created some hackery to work around my generated proto code not being visible in
/// my IDE. So here's how to continue maintaining this build script for the future:
///
/// If you change anything in frj_ngn.proto, then uncomment the line below, run
/// `cargo build && git add ./protobuf/frj_ngn.proto`, then comment the line again.
///
/// This isn't a necessarily happy end state, but I want to move on to solving other problems
/// for now.
fn main() -> io::Result<()> {
    //build_proto()?;
    Ok(())
}

#[allow(dead_code)]
fn build_proto() -> io::Result<()> {
    tonic_build::configure()
        .out_dir("./src/wire_api/")
        .build_client(true)
        .build_server(false)
        .compile(
            &["./../../protobuf/frj_ngn.proto"],
            &["./../../protobuf/"],
        )
}
