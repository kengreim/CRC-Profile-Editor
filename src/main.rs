use std::fs::File;
use std::io::Read;
use crate::crc_serde::CrcProfile;

mod crc_serde;

// fn test_deserialize(filename: &str) -> anyhow:E


fn main() -> std::io::Result<()> {
    //let mut x = File::open("nct_1.json")?;
    let mut x = File::open("zoa_1.json")?;
    let mut contents = String::new();
    x.read_to_string(&mut contents)?;

    // if let Err(e) = serde_json::from_str::<CrcProfile>(&contents) {
    //     println!("{}", e)
    // } else {
    //     println!("Success")
    // }


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

    Ok(())
}
