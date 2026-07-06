use pikaxe::io::{BinaryStream, FileStream, MiloArchive, PathFinder, Stream};
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{SeekFrom, Write};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("object_research [input_game_dir_path] [output_dir_path]");
        return;
    }

    let game_dir = PathBuf::from(&args[0]);
    let Ok(all_files) = game_dir.get_all_files() else {
        return;
    };

    let files = all_files
        .iter()
        .filter(|f| f
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.starts_with("milo")))
        .collect::<Vec<_>>();

    println!("Found {} milos", files.len());

    let mut dir_classes = HashMap::new();
    let mut milo_files = Vec::new();

    for file in files.iter() {
        let relative_path = file.strip_prefix(&game_dir).unwrap();
        println!("Opening {}", relative_path.to_string_lossy());

        let mut milo_archive = open_milo_archive(file);
        let Some((endian, _version)) = milo_archive.guess_endian_version() else {
            continue;
        };

        let mut stream = milo_archive.get_stream();

        let mut reader = BinaryStream::from_stream_with_endian(stream.as_mut(), endian);
        let uncompressed_file_size = reader.len().unwrap();

        reader.seek(SeekFrom::Start(4)).unwrap();

        let dir_class = reader.read_prefixed_string().unwrap();
        let dir_name = reader.read_prefixed_string().unwrap();
        reader.seek(SeekFrom::Current(8)).unwrap(); // Skip extra nums

        // Read entries
        let entry_count = reader.read_int32().unwrap();
        for _ in 0..entry_count {
            let entry_class = reader.read_prefixed_string().unwrap();
            let entry_name = reader.read_prefixed_string().unwrap();
        }

        let obj_dir_version = reader.read_uint32().unwrap();

        if !dir_classes.contains_key(&dir_class) {
            dir_classes.insert(dir_class.to_owned(), obj_dir_version);
        }

        milo_files.push((relative_path.to_string_lossy().to_string(), dir_class, obj_dir_version, uncompressed_file_size));
    }

    println!("Finished processing {} milos", files.len());

    let output_dir = PathBuf::from(&args[1]);

    let file_infos_path = output_dir.join("files.csv");
    write_file_info(&file_infos_path, &milo_files).unwrap();

    //let object_dirs_path = output_dir.join("object_dirs.csv");
    //write_object_dirs(&object_dirs_path, &dir_classes).unwrap();
}

fn open_milo_archive(path: &Path) -> MiloArchive {
    let mut fs = FileStream::from_path_as_read_open(path).unwrap();
    MiloArchive::from_stream(&mut fs).unwrap()
}

fn write_file_info(file_path: &Path, milo_files: &Vec<(String, String, u32, usize)>) -> std::io::Result<()> {
    let mut file = std::fs::File::create(file_path)?;

    writeln!(&mut file, "path,object_dir,version,size");

    for (milo_path, dir_name, dir_version, file_size) in milo_files.iter() {
        let formatted_path = milo_path.replace("(..)", "..");

        writeln!(&mut file, "{},{},{},{}", formatted_path, dir_name, dir_version, file_size);
    }

    Ok(())
}

fn write_object_dirs(file_path: &Path, dir_classes: &HashMap<String, u32>) -> std::io::Result<()> {
    let mut file = std::fs::File::create(file_path)?;

    let mut object_dir_classes = dir_classes.keys().collect::<Vec<_>>();
    object_dir_classes.sort();

    writeln!(&mut file, "object_dir,version");

    for object_dir_class in object_dir_classes.iter() {
        let Some(dir_version) = dir_classes.get(*object_dir_class) else {
            continue;
        };

        writeln!(&mut file, "{},{}", object_dir_class, *dir_version);
    }

    Ok(())
}