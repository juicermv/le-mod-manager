use crate::data::package::package_file_raw::PackageFileRaw;
use crate::data::package::package_header_raw::PackageHeaderRaw;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct PackageRaw {
    pub(crate) p_header: PackageHeaderRaw,
    pub(crate) p_files: Vec<PackageFileRaw>,
}