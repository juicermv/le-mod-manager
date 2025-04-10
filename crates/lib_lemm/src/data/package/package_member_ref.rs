use crate::data::PackageMemberType;


#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct PackageMemberRef {
    pub name: String,
    pub package_member_type: PackageMemberType,
}