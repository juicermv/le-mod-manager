use std::collections::VecDeque;
use crate::data::PackageMemberHeader;

pub struct FileQueue {
    pub(crate) q: VecDeque<(PackageMemberHeader, u64)>
}

impl FileQueue {
    pub(crate) fn deque (&mut self) -> Option<(PackageMemberHeader, u64)> {
        self.q.pop_front()
    }
}