export const PageState: any = {}
export const PageUrl: { [key: string]: string } = {
    findTreeUrl: "./tree.action",
    findPageUrl: "./page.action",
    findInfoUrl: "./info.action",
    saveUrl: "./save.action",
    removeUrl: "./remove.action",
    enableUrl: "./enable.action",
    disableUrl: "./disable.action",
    changeParentUrl: "./update/parent.action",
}
export const PageAction: any = {
    init: () => {

        // 树查询框
        const [treeSearchValue, setTreeSearchValue] = React.useState<any>("");
        PageState.treeSearchValue = treeSearchValue;
        PageState.setTreeSearchValue = setTreeSearchValue;

        // 树数据
        const [treeData, setTreeData] = React.useState<any>([]);
        PageState.treeData = treeData;
        PageState.setTreeData = setTreeData;

        // 树选中Key
        const [treeSelectedKeys, setTreeSelectedKeys] = React.useState<any>([]);
        PageState.treeSelectedKeys = treeSelectedKeys;
        PageState.setTreeSelectedKeys = setTreeSelectedKeys;
        // 树展开Key
        const [treeExpandedKeys, setTreeExpandedKeys] = React.useState<any>([]);
        PageState.treeExpandedKeys = treeExpandedKeys;
        PageState.setTreeExpandedKeys = setTreeExpandedKeys;

        // 树选中节点
        const [treeSelectedNodeData, setTreeSelectedNodeData] = React.useState<any>([]);
        PageState.treeSelectedNodeData = treeSelectedNodeData;
        PageState.setTreeSelectedNodeData = setTreeSelectedNodeData;

        // 查询表单数据
        const [searchFormData, setSearchFormData] = React.useState<any>({});
        PageState.searchFormData = searchFormData;
        PageState.setSearchFormData = setSearchFormData;
        PageState.searchFormRef = React.useRef();
        // 列表数据
        const [tableData, setTableData] = React.useState<any>([]);
        PageState.tableData = tableData;
        PageState.setTableData = setTableData;
        // 分页数据
        const [pageData, setPageData] = React.useState<any>({
            pageNum: 1,
            pageSize: 10,
            total: 0,
        });
        PageState.pageData = pageData;
        PageState.setPageData = setPageData;
        // 列表选中Key
        const [tableSelectedKeys, setTableSelectedKeys] = React.useState<any>([]);
        PageState.tableSelectedKeys = tableSelectedKeys;
        PageState.setTableSelectedKeys = setTableSelectedKeys;
        const [tableSelectedNode, setTableSelectedNode] = React.useState<any>([]);
        PageState.tableSelectedNode = tableSelectedNode;
        PageState.setTableSelectedNode = setTableSelectedNode;
        // 当前表单数据
        const [formData, setFormData] = React.useState<any>({});
        PageState.formData = formData;
        PageState.setFormData = setFormData;
        // 新增表单
        const [addFormVisible, setAddFormVisible] = React.useState<any>(false);
        PageState.addFormVisible = addFormVisible;
        PageState.setAddFormVisible = setAddFormVisible;
        PageState.addFormRef = React.useRef();
        // 编辑表单
        const [editFormVisible, setEditFormVisible] = React.useState<any>(false);
        PageState.editFormVisible = editFormVisible;
        PageState.setEditFormVisible = setEditFormVisible;
        PageState.editFormRef = React.useRef();
        // 详情表单
        const [detailFormVisible, setDetailFormVisible] = React.useState<any>(false);
        PageState.detailFormVisible = detailFormVisible;
        PageState.setDetailFormVisible = setDetailFormVisible;
        PageState.detailFormRef = React.useRef();
        // 变更所属表单
        const [changeOwnerFormVisible, setChangeOwnerFormVisible] = React.useState<any>(false);
        PageState.changeOwnerFormVisible = changeOwnerFormVisible;
        PageState.setChangeOwnerFormVisible = setChangeOwnerFormVisible;
        PageState.changeOwnerFormRef = React.useRef();
    },
    findTreeData: () => {
    },
    findPageData: () => {
    },
    findInfoData: () => {
    },
    saveData: (data: any, callback: any) => {
    },
    removeData: (data: any) => {
    },
    enableData: (data: any) => {
    },
    disableData: (data: any) => {
    },

    openAddBrotherForm: () => {
    },
    openAddChildForm: () => {
    },
    openEditForm: () => {
    },
    openInfoForm: () => {
    },
    openChangeParentForm: () => {
    },

}
