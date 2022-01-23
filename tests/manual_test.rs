//! Testing ground for parsers
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::io::Read;

use pk2::unsync::readonly::Pk2;

#[test]
#[ignore]
fn manual_test() {
    let pk2 = Pk2::open_readonly(
        dbg!(std::env::var("SR_FORMATS_TEST_PK2_PATH").unwrap()),
        "169841",
    )
    .unwrap();
    let mut exts = HashSet::new();
    let mut processed = HashMap::<_, u32>::new();
    pk2.for_each_file("/", |path, mut file| {
        let ext = match path.extension().and_then(OsStr::to_str) {
            Some(ext) => ext.to_lowercase(),
            None => {
                println!("Ignoring file without extension: {}", path.display());
                return Ok(());
            },
        };
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        if buf.is_empty() {
            return Ok(());
        }

        let res = match &*ext {
            "ifo" => return Ok(()),
            "bms" => sr_formats::jmxvbms::JmxBMesh::parse(&buf).map(drop),
            "bmt" => sr_formats::jmxvbmt::JmxMat::parse(&buf).map(drop),
            "ban" => sr_formats::jmxvban::JmxAnimation::parse(&buf).map(drop),
            "bsk" => sr_formats::jmxvbsk::JmxSkeleton::parse(&buf).map(drop),
            "bsr" => sr_formats::jmxvbsr::JmxRes::parse(&buf).map(drop),
            "cpd" => sr_formats::jmxvcpd::JmxCompound::parse(&buf).map(drop),
            "ddj" => sr_formats::jmxvddj::JmxTexture::parse(&buf).map(drop),
            "dof" => sr_formats::jmxvdof::JmxDungeon::parse(&buf).map(drop), // somethings very wrong here
            // "nvm" => sr_formats::jmxvnvm::JmxNvm::parse(&buf).map(drop), bugged 😬
            "mfo" => sr_formats::jmxvmfo::JmxMapInfo::parse(&buf).map(drop),
            "tga" | "txt" | "wav" => Ok(()),
            "vsh" | "psh" | "c" => Ok(()), // shader
            ext => {
                exts.insert(ext.to_owned());
                return Ok(());
            },
        };
        if let Err(
            nom::Err::Error(nom::error::Error { code, .. })
            | nom::Err::Failure(nom::error::Error { code, .. }),
        ) = res
        {
            println!(
                "{}({:?}): {:?}",
                path.display(),
                buf.get(..12)
                    .and_then(|it| { std::str::from_utf8(it).ok() }),
                code
            );
        } else {
            *processed.entry(ext).or_default() += 1;
        }
        Ok(())
    })
    .unwrap();
    print!("Unparsed extensions: ");
    exts.into_iter().for_each(|ext| print!("{}, ", ext));
    println!();
    println!("Parsed extensions:");
    processed
        .into_iter()
        .for_each(|(ext, count)| println!("{}: {}", ext, count));
}
