use anyhow::Result;

pub mod data;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::data::{Package, PackageFile};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn pkg_writing() -> Result<()> {
        let mut pkg = Package::new();
        pkg.set_mod_name("MOD NAME")?;
        pkg.set_mod_author("THE LORD")?;
        pkg.set_mod_version("1.22")?;

        pkg.add_file(PackageFile {
            name: "FILE1.ini".into(),
            file_type: data::PackageFileType::INI,
            data: "[core]\nnothing=true".as_bytes().to_vec(),
        })?;

        pkg.add_file(PackageFile {
            name: "FILE2.pkg".into(),
            file_type: data::PackageFileType::PKG,
            data: "NOT VALID DATA!".as_bytes().to_vec(),
        })?;

        pkg.write("./hello.lemm")?;

        let mut new_pkg = Package::from_disk("./hello.lemm")?;
        new_pkg.load_files()?;
        assert_eq!(new_pkg, pkg);

        Ok(())
    }
}
