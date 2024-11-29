use bmbp_abc::{BmbpTree, BmbpTreeModel};
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "BMBP_RBAC_APP_GROUP";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpAppGroup {
    #[serde(flatten)]
    pub tree: BmbpTree<BmbpAppGroup>,
}

impl BmbpTreeModel<BmbpAppGroup> for BmbpAppGroup {
    fn tree_mut(&mut self) -> &mut BmbpTree<BmbpAppGroup> {
        &mut self.tree
    }
    fn tree(&self) -> &BmbpTree<BmbpAppGroup> {
        &self.tree
    }
}
