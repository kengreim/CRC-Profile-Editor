use crate::crc_serde::CrcProfile;
use std::fs::{read_dir, File};
use std::io::Read;

mod crc_serde;

fn main() -> std::io::Result<()> {
    let mut profiles_dir = dirs::data_local_dir().unwrap();
    profiles_dir.push("CRC");
    profiles_dir.push("Profiles");

    for file in read_dir(&profiles_dir)? {
        let file = file?;
        if file.file_type().is_ok_and(|f| f.is_file())
            && file
                .file_name()
                .to_str()
                .is_some_and(|s| s.ends_with(".json"))
        {

            let filename = file.file_name();
            println!("{:?}", filename);

            let mut file_path = profiles_dir.clone();
            file_path.push(filename);

            let mut x = File::open(file_path)?;
            let mut contents = String::new();
            x.read_to_string(&mut contents)?;
            let jd = &mut serde_json::Deserializer::from_str(&contents);
            let result: Result<CrcProfile, _> = serde_path_to_error::deserialize(jd);

            match result {
                Ok(_) => println!("success"),
                Err(err) => {
                    let path = err.path().to_string();
                    println!("{path}");
                    println!("{:?}", err)
                }
            }
        }
    }
    //
    // //let mut x = File::open("nct_1.json")?;
    // let mut x = File::open("zoa_1.json")?;
    // let mut contents = String::new();
    // x.read_to_string(&mut contents)?;
    //
    // // if let Err(e) = serde_json::from_str::<CrcProfile>(&contents) {
    // //     println!("{}", e)
    // // } else {
    // //     println!("Success")
    // // }
    //
    // let jd = &mut serde_json::Deserializer::from_str(&contents);
    // let result: Result<CrcProfile, _> = serde_path_to_error::deserialize(jd);
    // match result {
    //     Ok(_) => println!("success"),
    //     Err(err) => {
    //         let path = err.path().to_string();
    //         println!("{path}");
    //         println!("{:?}", err)
    //     }
    // }

    Ok(())
}
