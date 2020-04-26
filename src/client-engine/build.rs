use common_build::{BuildConfig, BuildMode};
use std::io;
use std::path::Path;

const PROTO_FILE: &str = "./../../protobuf/frj_ngn.proto";
const HASH_FILE: &str = "./.proto_hash";

/// Generate protobuf code in the `src/` directory. This is unusual, but otherwise
/// the code that's generated doesn't show up in my IDE (which hinders my speed of
/// learning).
///
/// This build script automatically detects changes in the .proto file and only
/// re-generates the src code if .proto changes. This won't work for all cases,
/// e.g. if we change our tonic version without changing the .proto file.
///
/// To force a clean generation of protobuf code, delete the `.proto_hash` file.
fn main() -> io::Result<()> {
    common_build::do_it(BuildConfig {
        mode: BuildMode::Client,
        proto_file_path: Path::new(PROTO_FILE),
        hash_file_path: Path::new(HASH_FILE),
    })
}
