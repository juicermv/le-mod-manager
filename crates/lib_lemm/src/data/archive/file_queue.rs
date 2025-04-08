use crate::data::PackageMemberHeader;
use std::collections::VecDeque;

pub struct FileQueue {
    pub(crate) q: VecDeque<(PackageMemberHeader, u64)>,
}

impl FileQueue {
    pub(crate) fn deque(&mut self) -> Option<(PackageMemberHeader, u64)> {
        self.q.pop_front()
    }

    pub fn filter(&self, mut lambda: impl FnMut(&PackageMemberHeader) -> bool) -> Self {
        Self {
            q: self.q.iter().filter(|item| lambda(&item.0)).cloned().collect(),
        }
    }
}
