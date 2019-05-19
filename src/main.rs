#![feature(try_blocks)]

use std::path::PathBuf;

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
    use glob::glob;

    //let mut mats = Vec::with_capacity(100);
    for entry in glob("D:/Silkroad/Data/navmesh/**/*.nvm").expect("Failed to read glob pattern") {
        let path = match entry {
            Ok(path) => path,
            Err(e) => continue,
        };
        let res: Result<_, Box<dyn std::error::Error>> = try {
            let buf = std::fs::read(&path)?;
            sr_formats::jmxvnvm::JmxNvm::parse(&buf).map_err(|e| format!("{:?}", e))?
        };
        match res {
            Ok(mut mat) => (), //mats.append(&mut mat.0),
            Err(e) => println!("{:?}", path),
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
