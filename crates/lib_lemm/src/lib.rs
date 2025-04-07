use anyhow::Result;

pub mod data;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        to_ascii_array, PackageHeader, PackageMemberHeader, PackageMemberType, PackageReader,
        PackageWriter,
    };
    use ascii::AsciiChar::r;
    use ascii::{AsAsciiStr, AsciiChar};
    use flate2::read::{GzDecoder, GzEncoder, ZlibEncoder};
    use std::fmt::Debug;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, Write};
    use std::path::PathBuf;
    use text_io::read;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn pkg_string_test() -> Result<()> {
        let writer = PackageWriter::new("./hello.lemm".into());
        let header = PackageHeader {
            mod_name: to_ascii_array("An LE mod"),
            mod_author: to_ascii_array("The Lord"),
            mod_version: to_ascii_array("1.21"),
        };

        let files: Vec<(PackageMemberHeader, Vec<u8>)> = vec![
            (
                PackageMemberHeader {
                    file_type: PackageMemberType::INI,
                    file_name: to_ascii_array("mod.ini"),
                    content_length: 0,
                },
                "Hello".as_bytes().to_vec(),
            ),
            (
                PackageMemberHeader {
                    file_type: PackageMemberType::CONFIG,
                    file_name: to_ascii_array("mod_cfg.cfg"),
                    content_length: 0,
                },
                "This is a config!".as_bytes().to_vec(),
            ),
        ];

        writer.write(&header, &files)?;

        let reader = PackageReader::new("./hello.lemm".into());
        let read_header = reader.read_header()?;
        let read_member_headers = reader.read_member_headers()?;
        let read_member_data = reader.read_member_contents(read_member_headers)?;

        for (i, (header, data)) in read_member_data.iter().enumerate() {
            assert_eq!(
                String::from_utf8(files[i].1.clone()),
                String::from_utf8(data.clone())
            );
        }

        Ok(())
    }

    #[test]
    fn pkg_files_test() -> Result<()> {
        let files = vec![
            "./src/data/package/package_reader.rs",
             "../lemm_app/Dioxus.toml"
        ];

        let mut writer = PackageWriter::new("./hello2.lemm".into());
        let header = PackageHeader {
            mod_name: to_ascii_array("An LE mod"),
            mod_author: to_ascii_array("juicermv"),
            mod_version: to_ascii_array("1.21"),
        };

        writer.write(&header, &vec![])?;

        for file_name in files {
            if fs::exists(&file_name)? && fs::metadata(&file_name)?.is_file() {
                let file = File::open(&file_name)?;

                let mut compressor = GzEncoder::new(file, flate2::Compression::default());

                let mut buffer = Vec::new();
                compressor.read_to_end(&mut buffer)?;

                let path: PathBuf = file_name.into();

                writer.append(&vec![(
                    PackageMemberHeader {
                        file_type: PackageMemberType::ETexture,
                        file_name: to_ascii_array(path.file_name().unwrap().to_str().unwrap()),
                        content_length: buffer.len() as u64,
                    },
                    buffer,
                )])?
            }
        }

        let reader = PackageReader::new("./hello2.lemm".into());
        let read_header = reader.read_header()?;
        let read_member_headers = reader.read_member_headers()?;
        for (header, _pos) in read_member_headers {
            let file_name = header.file_name.as_ascii_str().unwrap();
            let file_type = header.file_type.clone();
            println!("File name: {}, File type: {:?}", file_name, file_type);

            let content: &[u8] = &reader.read_member_contents(vec![(header, _pos)])?[0].1;
            let mut decoder = GzDecoder::new(content);

            let mut buffer: Vec<u8> = Vec::new();
            decoder.read_to_end(&mut buffer)?;

            println!("buffer: {}", String::from_utf8_lossy(&buffer));
        }

        Ok(())
    }
}
