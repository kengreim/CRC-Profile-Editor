#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::ffi::OsString;
use crate::crc_serde::CrcProfile;
use std::fs::{read_dir, File};
use std::io::Read;
use std::path::Path;

mod crc_serde;

fn extension_matches(path: &OsString, extension: &str) -> bool {
    Path::new(path)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case(extension))
}

fn main() -> std::io::Result<()> {
    let mut profiles_dir = dirs::data_local_dir().unwrap();
    profiles_dir.push("CRC");
    profiles_dir.push("Profiles");

    for file in read_dir(&profiles_dir)? {
        match file {
            Err(e) => println!("{e:?}"),
            Ok(file) => {
                if file.file_type().is_ok_and(|f| f.is_file())
                    && extension_matches(&file.file_name(), "json")
                {
                    println!("{:?}", file.file_name());
                    let mut x = File::open(file.path())?;
                    let mut contents = String::new();
                    x.read_to_string(&mut contents)?;
                    let jd = &mut serde_json::Deserializer::from_str(&contents);
                    let result: Result<CrcProfile, _> = serde_path_to_error::deserialize(jd);

                    match result {
                        Ok(_) => println!("success"),
                        Err(err) => {
                            let path = err.path().to_string();
                            println!("{path}");
                            println!("{err:?}");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
