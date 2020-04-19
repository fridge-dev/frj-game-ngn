use std::io;

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
    cached_code_generation::generate_code(PROTO_FILE, HASH_FILE, build_proto)
}

#[allow(dead_code)]
fn build_proto() -> io::Result<()> {
    tonic_build::configure()
        .out_dir("./src/wire_api/")
        .build_client(false)
        .build_server(true)
        .compile(
            &["./../../protobuf/frj_ngn.proto"],
            &["./../../protobuf/"],
        )
}

/// TODO move to a common place, as this is duplicated to client-engine/build.rs
mod cached_code_generation {
    use std::io;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Write, SeekFrom, Seek};

    pub fn generate_code<F>(
        source_file_path: &str,
        hash_file_path: &str,
        generator: F
    ) -> io::Result<()>
        where
            F: FnOnce() -> io::Result<()>
    {
        // 1. Get the hash of the .proto file contents
        println!("Opening {}", source_file_path);
        let mut source_file = File::open(source_file_path)?;

        println!("Reading contents of {}", source_file_path);
        let source_file_contents = read_file_contents(&mut source_file)?;

        println!("Hashing contents of {}", source_file_path);
        let computed_hashed = hash(&source_file_contents);

        // 2. Get existing hash value
        println!("Opening {}", hash_file_path);
        let mut hash_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(hash_file_path)?;

        println!("Reading contents of {}", hash_file_path);
        let existing_hash = read_file_contents(&mut hash_file)?;

        // 3. Compare hashes and generate proto code
        println!("Computed hash: '{:?}' ; Existing hash: '{:?}'", computed_hashed, existing_hash);
        if computed_hashed == existing_hash {
            println!(">>> Hashes are equal. Doing nothing.");
        } else {
            println!(">>> Hashes are NOT equal. Running code generator.");
            generator()?;

            println!("Resetting hash file cursor to beginning.");
            hash_file.seek(SeekFrom::Start(0))?;
            hash_file.set_len(0)?;

            println!("Saving new hash to file.");
            hash_file.write(&computed_hashed)?;
        }

        Ok(())
    }

    fn read_file_contents(file: &mut File) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn hash<T: Hash>(obj: &T) -> Vec<u8> {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        let hash: u64 = hasher.finish();

        vec![
            hash as u8,
            (hash >> 8) as u8,
            (hash >> 16) as u8,
            (hash >> 24) as u8,
            (hash >> 32) as u8,
            (hash >> 40) as u8,
            (hash >> 48) as u8,
            (hash >> 56) as u8,
        ]
    }
}
