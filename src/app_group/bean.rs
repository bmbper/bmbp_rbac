use bmbp_abc::{BmbpTree, BmbpTreeModel};
use bmbp_marco_table::table;
use serde::{Deserialize, Serialize};

#[table(name = "BMBP_RBAC_APP_GROUP")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BmbpAppGroup {
    #[serde(flatten)]
    pub group: BmbpTree<BmbpAppGroup>,
}

impl BmbpTreeModel<BmbpAppGroup> for BmbpAppGroup {
    fn tree_mut(&mut self) -> &mut BmbpTree<BmbpAppGroup> {
        &mut self.group
    }
    fn tree(&self) -> &BmbpTree<BmbpAppGroup> {
        &self.group
    }
}
