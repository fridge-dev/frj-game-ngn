use std::path::Path;
use std::io;

pub struct BuildConfig<'a> {
    pub mode: BuildMode,
    pub proto_file_path: &'a Path,
    pub hash_file_path: &'a Path,
}

pub enum BuildMode {
    Client,
    Server,
}

pub fn do_it(config: BuildConfig) -> io::Result<()> {
    let proto_file_path_str = config.proto_file_path.to_str().expect("Non unicode file path");
    let hash_file_path_str = config.hash_file_path.to_str().expect("Non unicode file path");
    cached_code_generation::generate_code(proto_file_path_str, hash_file_path_str, || {
        grpc_compiler::build_proto(config.mode, config.proto_file_path)
    })
}

pub mod grpc_compiler {
    use std::io;
    use std::path::Path;
    use crate::BuildMode;

    pub fn build_proto(mode: BuildMode, proto_path: &Path) -> io::Result<()> {
        let proto_dir = proto_path.parent().expect("file must be within a directory wtf");

        let builder = match mode {
            BuildMode::Client => tonic_build::configure().build_server(false).build_client(true),
            BuildMode::Server => tonic_build::configure().build_server(true).build_client(false),
        };

        builder
            .out_dir("./src/wire_api/")
            .compile(
                &[proto_path],
                &[proto_dir],
            )
    }
}

pub mod cached_code_generation {
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
        // TODO also hash entire directory of `source_file_path`

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
