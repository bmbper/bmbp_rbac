// 全局状态声明
export const PageState: any = {};

export const PageUrl = {
  findTreeUrl: "./tree",
  findIgnoreTreeUrl: "./tree/ignore",
  findPageUrl: "./page",
  saveUrl: "./save",
  findInfoUrl: "./info",
  removeUrl: "./remove",
  enableUrl: "./enable",
  disableUrl: "./disable",
  updateParentUrl: "./update/parent",
};
// 全局方法
export const PageAction = {
  init: (props: any) => {
    // 参数树数据
    const [treeData, setTreeData] = React.useState([]);
    PageState.treeData = treeData;
    PageState.setTreeData = setTreeData;

    // 参数树选中节点数据
    const [selectTreeNodeData, setSelectTreeNodeData] = React.useState();
    PageState.selectTreeNodeData = selectTreeNodeData;
    PageState.setSelectTreeNodeData = setSelectTreeNodeData;

    // 参数树选中节点KEY
    const [selectTreeNodeKeys, setSelectTreeNodeKeys] = React.useState([]);
    PageState.selectTreeNodeKeys = selectTreeNodeKeys;
    PageState.setSelectTreeNodeKeys = setSelectTreeNodeKeys;

    // 参数列表-查询数据
    const [searchFormData, setSearchFormData] = React.useState({});
    PageState.setSearchFormData = setSearchFormData;
    PageState.searchFormData = searchFormData;
    // 查询表单关联关系
    PageState.searchFormRef = React.useRef(null);

    // 选中表格key，选中表格行
    const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
    PageState.selectedRowKeys = selectedRowKeys;
    PageState.setSelectedRowKeys = setSelectedRowKeys;
    const [selectedRows, setSelectedRows] = React.useState([]);
    PageState.selectedRows = selectedRows;
    PageState.setSelectedRows = setSelectedRows;

    // 分页可选项
    const [showPageSize, setShowPageSize] = React.useState("default");
    PageState.showPageSize = showPageSize;
    PageState.setShowPageSize = setShowPageSize;
    // 列表数据
    const [gridData, setGridData] = React.useState([]);
    PageState.gridData = gridData;
    PageState.setGridData = setGridData;
    //分页数据
    const [pageData, setPageData] = React.useState({
      pageNo: 1,
      pageSize: 10,
      total: 0,
    });
    PageState.pageData = pageData;
    PageState.setPageData = setPageData;
    const [currentOrganData, setCurrentOrganData] = React.useState({});
    PageState.currentOrganData = currentOrganData;
    PageState.setCurrentOrganData = setCurrentOrganData;
    // 新增参数
    const [addFormDialogVisible, setAddFormDialogVisible] =
      React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    // 编辑参数
    const [editFormDialogVisible, setEditFormDialogVisible] =
      React.useState(false);
    PageState.editFormDialogVisible = editFormDialogVisible;
    PageState.setEditFormDialogVisible = setEditFormDialogVisible;
    // 查看参数
    const [infoFormDialogVisible, setInfoFormDialogVisible] =
      React.useState(false);
    PageState.infoFormDialogVisible = infoFormDialogVisible;
    PageState.setInfoFormDialogVisible = setInfoFormDialogVisible;
    // 变更上级
    const [changeParentFormDialogVisible, setChangeParentFormDialogVisible] =
      React.useState(false);
    PageState.changeParentFormDialogVisible = changeParentFormDialogVisible;
    PageState.setChangeParentFormDialogVisible =
      setChangeParentFormDialogVisible;
    // 待变更的参数
    const [waitChangeOrganDataId, setWaitChangeOrganDataId] =
      React.useState("");
    PageState.waitChangeOrganDataId = waitChangeOrganDataId;
    PageState.setWaitChangeOrganDataId = setWaitChangeOrganDataId;
    const [parentTreeData, setParentTreeData] = React.useState([]);
    PageState.parentTreeData = parentTreeData;
    PageState.setParentTreeData = setParentTreeData;
    // 改变排序
    const [
      changeShowOrderFormDialogVisible,
      setChangeShowOrderFormDialogVisible,
    ] = React.useState(false);
    PageState.changeShowOrderFormDialogVisible =
      changeShowOrderFormDialogVisible;
    PageState.setChangeShowOrderFormDialogVisible =
      setChangeShowOrderFormDialogVisible;
    // 导入功能
    const [importFormDialogVisible, setImportFormDialogVisible] =
      React.useState(false);
    PageState.importFormDialogVisible = importFormDialogVisible;
    PageState.setImportFormDialogVisible = setImportFormDialogVisible;
    // 导出功能
    const [exportFormDialogVisible, setExportFormDialogVisible] =
      React.useState(false);
    PageState.exportFormDialogVisible = exportFormDialogVisible;
    PageState.setExportFormDialogVisible = setExportFormDialogVisible;

    PageState.addFormRef = React.useRef();
    PageState.editFormRef = React.useRef();
    PageState.infoFormRef = React.useRef();
    PageState.changeParentFormRef = React.useRef();
    PageState.showOrderFormRef = React.useRef();
    PageState.importFormRef = React.useRef();
    PageState.exportFormRef = React.useRef();
  },
  findTreeData: (v: String | null) => {
    if (!v && v == "") {
      PageState.setSelectTreeNodeKeys([]);
      PageState.setSelectTreeNodeData({});
    }
    axios
      .post(PageUrl.findTreeUrl, { organName: v })
      .then((resp: any) => {
        const { code, msg, data } = resp;
        if (code == 0) {
          PageState.setTreeData(data);
          PageAction.findGridData();
        } else {
          console.log("error:", resp);
          arco.Message.error(resp.msg);
        }
      })
      .catch((err: any) => {
        console.log("error:", err);
        arco.Message.error("系统好像是走丢了，请联系管理员");
      });
  },
  findTreeDataIgnoreDataId: (dataId: String) => {
    axios
      .post(PageUrl.findIgnoreTreeUrl, { dataId: dataId })
      .then((resp: any) => {
        const { code, msg, data } = resp;
        if (code == 0) {
          PageState.setParentTreeData(data);
        } else {
          console.log("error:", resp);
          arco.Message.error(resp.msg);
        }
      })
      .catch((err: any) => {
        console.log("error:", err);
        arco.Message.error("系统好像是走丢了，请联系管理员");
      });
  },
  findGridData: () => {
    let searchFormData = PageState.searchFormData;
    let pageParams = {
      pageNo: PageState.pageData.pageNo,
      pageSize: PageState.pageData.pageSize,
      params: {
        organParentCode: PageState.selectTreeNodeData?.organCode,
        ...searchFormData,
      },
    };
    axios
      .post(PageUrl.findPageUrl, pageParams)
      .then((resp: any) => {
        const { code, msg, data } = resp;
        if (code == 0) {
          PageState.setGridData(data.data);
          PageState.setPageData({ ...PageState.pageData, total: data.total });
        } else {
          console.log("error:", resp);
          arco.Message.error(resp.msg);
        }
      })
      .catch((err: any) => {
        console.log("error:", err);
        arco.Message.error("系统好像是走丢了，请联系管理员");
      });
  },
  addBrotherNode: (node: any) => {
    let organParentCode = node.organParentCode;
    let currentData = {
      organParentCode: organParentCode,
    };
    PageState.setCurrentOrganData(currentData);
    PageState.setAddFormDialogVisible(true);
  },
  addChildNode: (node: any) => {
    let organParentCode = "#";
    if (node && node.organCode) {
      organParentCode = node.organCode;
    } else {
      if (
        PageState.selectTreeNodeData &&
        PageState.selectTreeNodeData.organCode
      ) {
        organParentCode = PageState.selectTreeNodeData.organCode;
      }
    }
    let currentData = {
      organParentCode: organParentCode,
    };
    PageState.setCurrentOrganData(currentData);
    PageState.setAddFormDialogVisible(true);
  },
  editNode: (node: any) => {
    let dataId = node.dataId;
    axios
      .post(PageUrl.findInfoUrl + "?dataId=" + dataId, {})
      .then((resp: any) => {
        if (resp.code == 0) {
          PageState.setCurrentOrganData(resp.data);
          PageState.setEditFormDialogVisible(true);
        } else {
          arco.Message.error(resp.msg);
        }
      });
  },
  save(organData: any, callback: () => void) {
    axios.post(PageUrl.saveUrl, organData).then((resp: any) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        callback();
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  updateParent(organData: any, callback: () => void) {
    axios.post(PageUrl.updateParentUrl, organData).then((resp: any) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        callback();
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  removeNode: (node: any) => {
    axios
      .post(PageUrl.removeUrl + "?dataId=" + node.dataId, {})
      .then((resp: any) => {
        if (resp.code == 0) {
          arco.Message.success(resp.msg);
          PageAction.findTreeData(null);
        } else {
          arco.Message.error(resp.msg);
        }
      });
  },
  batchRemoveNode: (keys: String[]) => {
    console.log("delOrgan");
  },
  enableNode: (node: any) => {
    axios
      .post(PageUrl.enableUrl + "?dataId=" + node.dataId, {})
      .then((resp: any) => {
        if (resp.code == 0) {
          arco.Message.success(resp.msg);
          PageAction.findTreeData(null);
        } else {
          arco.Message.error(resp.msg);
        }
      });
  },
  disableNode: (node: any) => {
    axios
      .post(PageUrl.disableUrl + "?dataId=" + node.dataId, {})
      .then((resp: any) => {
        if (resp.code == 0) {
          arco.Message.success(resp.msg);
          PageAction.findTreeData(null);
        } else {
          arco.Message.error(resp.msg);
        }
      });
  },
  changeParentNode: (node) => {
    PageState.setWaitChangeOrganDataId(node.dataId);
    PageState.setChangeParentFormDialogVisible(true);
  },

  viewInfo(node: any) {
    let dataId = node.dataId;
    axios
      .post(PageUrl.findInfoUrl + "?dataId=" + dataId, {})
      .then((resp: any) => {
        if (resp.code == 0) {
          PageState.setCurrentOrganData(resp.data);
          PageState.setInfoFormDialogVisible(true);
        } else {
          arco.Message.error(resp.msg);
        }
      });
  },
};
