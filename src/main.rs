use glob::glob;
use std::{ffi::OsStr, path::PathBuf};

fn main() {
    /*
    let buf =
        std::fs::read("D:/Silkroad\\Data\\prim\\mtrl\\artifact\\china\\jangan\\cj_table_chair.bmt")
            .unwrap();
    let t = sr_formats::jmxvbmt_0102::JmxMat::parse(&buf).unwrap(); //println!("{:?}", sr_formats::jmxvbms_0110::JmxBMesh::parse(&buf));
    csv::WriterBuilder::new()
        .has_headers(false)
        .from_path("mats.csv")
        .unwrap()
        .serialize(&t.0)
        .unwrap();
    return;*/

    //let mut mats = Vec::with_capacity(100);
    use sr_formats::*;
    let exts = [
        "bms", "dof", "bmt", "ban", "bsk", "bsr", "cpd", "cpd", "nvm",
    ];
    for entry in glob("D:/Silkroad/Data/**/*").expect("Failed to read glob pattern") {
        let path: PathBuf = match entry {
            Ok(path) => path,
            Err(e) => continue,
        };
        let ext = path.extension().and_then(OsStr::to_str);
        let ext = if let Some(ext) = ext {
            if exts.iter().position(|ext2| *ext2 == ext).is_some() {
                ext
            } else {
                continue;
            }
        } else {
            continue;
        };
        let buf = std::fs::read(&path).unwrap();
        if buf.len() == 0 {
            continue;
        }
        let res = match ext {
            "bms" => None,
            "dof" => None,
            "bmt" => jmxvbmt::JmxMat::parse(&buf).err(),
            "ban" => None,
            "bsk" => jmxvbsk::JmxSkeleton::parse(&buf).err(),
            "bsr" => jmxvbsr::JmxRes::parse(&buf).err(),
            "cpd" => jmxvcpd::JmxCompound::parse(&buf).err(),
            "nvm" => None,
            _ => None,
        };
        match res {
            Some(_e) => println!("{:?}", path),
            None => (),
        }
    }
    /*
    csv::WriterBuilder::new()
        .has_headers(false)
        .from_path("mats.csv")
        .unwrap()
        .serialize(&mats)
        .unwrap();*/
}
