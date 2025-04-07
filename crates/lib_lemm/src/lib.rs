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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn pkg_writing() -> Result<()> {
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
}
