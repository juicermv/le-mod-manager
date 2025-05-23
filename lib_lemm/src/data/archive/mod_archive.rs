use crate::data::archive::file_queue::FileQueue;
use crate::data::package::PackageMemberRef;
use crate::data::{
    from_ascii_array, to_ascii_array, PackageHeader, PackageMemberHeader, PackageMemberType,
    PackageReader, PackageWriter,
};
use anyhow::Result;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct ModArchive {
    pkg_header: PackageHeader,
    pkg_members: Vec<(PackageMemberHeader, u64)>,
    pkg_path: PathBuf,
}

impl ModArchive {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let reader = PackageReader::new(path.as_ref().to_path_buf());

        Ok(Self {
            pkg_header: reader.read_header()?,
            pkg_members: reader.read_member_headers()?,
            pkg_path: path.as_ref().to_path_buf(),
        })
    }

    pub fn create(
        path: impl AsRef<Path>,
        mod_name: String,
        mod_author: String,
        mod_version: String,
    ) -> Result<Self> {
        let pkg_header = PackageHeader {
            mod_name: to_ascii_array(&mod_name),
            mod_author: to_ascii_array(&mod_author),
            mod_version: to_ascii_array(&mod_version),
        };

        let pkg_writer = PackageWriter::new(path.as_ref().to_path_buf());
        pkg_writer.write(&pkg_header, &vec![])?;

        Ok(Self {
            pkg_header,
            pkg_members: vec![],
            pkg_path: path.as_ref().to_path_buf(),
        })
    }

    pub fn update_header(
        &mut self,
        mod_name: String,
        mod_author: String,
        mod_version: String,
    ) -> Result<()> {
        self.pkg_header.mod_name = to_ascii_array(&mod_name);
        self.pkg_header.mod_author = to_ascii_array(&mod_author);
        self.pkg_header.mod_version = to_ascii_array(&mod_version);

        let pkg_writer = PackageWriter::new(self.pkg_path.clone());

        pkg_writer.write_header(&self.pkg_header)
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.pkg_path
    }

    pub fn get_mod_name(&self) -> String {
        from_ascii_array(&self.pkg_header.mod_name)
    }

    pub fn get_mod_author(&self) -> String {
        from_ascii_array(&self.pkg_header.mod_author)
    }

    pub fn get_mod_version(&self) -> String {
        from_ascii_array(&self.pkg_header.mod_version)
    }

    pub fn add_file(&mut self, path: impl AsRef<Path>, file_type: PackageMemberType) -> Result<()> {
        let file = File::open(&path)?;
        let mut compressed_file: Vec<u8> = Vec::new();

        GzEncoder::new(file, Compression::best()).read_to_end(&mut compressed_file)?;
        let header = PackageMemberHeader {
            file_type,
            file_name: to_ascii_array(path.as_ref().file_name().unwrap().to_str().unwrap()),
            content_length: compressed_file.len() as u64,
        };

        let pkg_writer = PackageWriter::new(self.pkg_path.clone());
        self.pkg_members
            .push(pkg_writer.append(&vec![(header.clone(), compressed_file)])?[0].clone());
        Ok(())
    }
    pub fn get_file_queue(&self, file_type: PackageMemberType) -> FileQueue {
        FileQueue {
            q: VecDeque::from(
                self.pkg_members
                    .iter()
                    .filter(|(header, _)| header.file_type == file_type)
                    .map(|val| (val.0.clone(), val.1))
                    .collect::<Vec<(PackageMemberHeader, u64)>>()
                    .clone(),
            ),
        }
    }

    pub fn get_refs(&self) -> Vec<PackageMemberRef> {
        self.pkg_members
            .iter()
            .map(|(header, _)| PackageMemberRef {
                name: from_ascii_array(&header.file_name),
                package_member_type: header.file_type,
            })
            .collect()
    }

    pub fn read_file_from_ref(&self, file_ref: &PackageMemberRef) -> Result<Option<Vec<u8>>> {
        let filtered_list = self
            .pkg_members
            .iter()
            .filter(|(header, _)| {
                header.file_type == file_ref.package_member_type
                    && from_ascii_array(&header.file_name) == file_ref.name
            })
            .collect::<Vec<&(PackageMemberHeader, u64)>>();

        if filtered_list.is_empty() {
            return Ok(None);
        }

        let header = filtered_list[0].clone();
        let reader = PackageReader::new(self.pkg_path.clone());
        let (_, contents_compressed) = &reader.read_member_contents(vec![header.clone()])?[0];

        let mut contents_decompressed: Vec<u8> = Vec::new();
        GzDecoder::new(contents_compressed.as_slice()).read_to_end(&mut contents_decompressed)?;

        Ok(Some(contents_decompressed))
    }

    pub fn get_file(&self, queue: &mut FileQueue) -> Result<Option<(String, Vec<u8>)>> {
        match queue.deque() {
            None => Ok(None),
            Some(header) => {
                let reader = PackageReader::new(self.pkg_path.clone());
                let (_, contents_compressed) =
                    &reader.read_member_contents(vec![header.clone()])?[0];

                let mut contents_decompressed: Vec<u8> = Vec::new();
                GzDecoder::new(contents_compressed.as_slice())
                    .read_to_end(&mut contents_decompressed)?;

                Ok(Some((
                    from_ascii_array(&header.0.file_name),
                    contents_decompressed,
                )))
            }
        }
    }

    pub fn clear_files(&mut self) -> Result<()> {
        self.pkg_members.clear();
        let pkg_writer = PackageWriter::new(self.pkg_path.clone());

        pkg_writer.write(&self.pkg_header, &Vec::new())
    }

    pub fn get_header(&self) -> &PackageHeader {
        &self.pkg_header
    }
}
