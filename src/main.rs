use std::fs::{self, File};
use std::io::{Error, ErrorKind, Write, BufWriter};
use std::path::Path;
use rfd::FileDialog;
use serde_json::json;
use zip::write::FileOptions;
use walkdir::WalkDir;

fn main() -> Result<(), Error> {
    fs::create_dir_all("custom-warning/assets/minecraft/sounds/custom")?;
    println!("\nDirectory created successfully");

    let mut pack_mcmeta_file = File::create("custom-warning/pack.mcmeta")?;
    let pack_mcmeta_content = r#"{
  "pack": {
    "pack_format": 1,
    "description": "custom sounds pack, autogenerated by @not_a_cow"
  }
}
"#;
    pack_mcmeta_file.write_all(pack_mcmeta_content.as_bytes())?;
    println!("pack.mcmeta created successfully");

    let files = FileDialog::new()
        .add_filter("OGG Files", &["ogg"])
        .pick_files()
        .expect("No files selected");

    let mut sounds_json = json!({});

    for file_path in files {
        let file_name = file_path.file_name().unwrap();
        let dest_path = Path::new("custom-warning/assets/minecraft/sounds/custom").join(file_name);
        fs::copy(&file_path, &dest_path)?;
        println!("Copied {} to {}", file_path.display(), dest_path.display());

        let sound_name = file_name.to_str().unwrap().replace(".ogg", "");
        sounds_json[format!("custom.{}", sound_name)] = json!({
            "sounds": [format!("custom/{}", sound_name)]
        });
    }

    let sounds_file = File::create("custom-warning/assets/minecraft/sounds/sounds.json")?;
    serde_json::to_writer_pretty(sounds_file, &sounds_json)?;
    println!("sounds.json created successfully");

    let zip_file = File::create("custom-warning.zip")?;
    let mut zip = zip::ZipWriter::new(BufWriter::new(zip_file));
    let options: FileOptions<'_, ()> = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for entry in WalkDir::new("custom-warning") {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(Path::new("custom-warning"))
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }

    zip.finish()?;
    println!("custom-warning.zip created successfully");

    Ok(())
}