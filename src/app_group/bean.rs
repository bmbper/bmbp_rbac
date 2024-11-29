use bmbp_abc::{BmbpTree, BmbpTreeModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpAppGroup {
    pub tree: BmbpTree<BmbpAppGroup>,
}

impl BmbpTreeModel<BmbpAppGroup> for BmbpAppGroup {
    fn tree_mut(&mut self) -> &mut BmbpTree<BmbpAppGroup> {
        self.tree_mut()
    }

    fn tree(&self) -> &BmbpTree<BmbpAppGroup> {
        self.tree()
    }
}
