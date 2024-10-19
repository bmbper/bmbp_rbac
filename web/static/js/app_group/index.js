// web/tsx/app_group/action.tsx
var PageState = {};
var PageUrl = {
  findTreeUrl: "./tree",
  findIgnoreTreeUrl: "./tree/ignore",
  findPageUrl: "./page",
  saveUrl: "./save",
  findInfoUrl: "./info",
  removeUrl: "./remove",
  batchRemoveUrl: "./batch/remove",
  enableUrl: "./enable",
  batchEnableUrl: "./batch/enable",
  disableUrl: "./disable",
  batchDisableUrl: "./batch/disable",
  updateParentUrl: "./update/parent"
};
var PageAction = {
  init: (props) => {
    const [treeData, setTreeData] = React.useState([]);
    PageState.treeData = treeData;
    PageState.setTreeData = setTreeData;
    const [selectTreeNodeData, setSelectTreeNodeData] = React.useState();
    PageState.selectTreeNodeData = selectTreeNodeData;
    PageState.setSelectTreeNodeData = setSelectTreeNodeData;
    const [selectTreeNodeKeys, setSelectTreeNodeKeys] = React.useState([]);
    PageState.selectTreeNodeKeys = selectTreeNodeKeys;
    PageState.setSelectTreeNodeKeys = setSelectTreeNodeKeys;
    const [searchFormData, setSearchFormData] = React.useState({});
    PageState.setSearchFormData = setSearchFormData;
    PageState.searchFormData = searchFormData;
    PageState.searchFormRef = React.useRef(null);
    const [selectedRowKeys, setSelectedRowKeys] = React.useState([]);
    PageState.selectedRowKeys = selectedRowKeys;
    PageState.setSelectedRowKeys = setSelectedRowKeys;
    const [selectedRows, setSelectedRows] = React.useState([]);
    PageState.selectedRows = selectedRows;
    PageState.setSelectedRows = setSelectedRows;
    const [showPageSize, setShowPageSize] = React.useState("default");
    PageState.showPageSize = showPageSize;
    PageState.setShowPageSize = setShowPageSize;
    const [gridData, setGridData] = React.useState([]);
    PageState.gridData = gridData;
    PageState.setGridData = setGridData;
    const [pageData, setPageData] = React.useState({
      pageNo: 1,
      pageSize: 10,
      total: 0
    });
    PageState.pageData = pageData;
    PageState.setPageData = setPageData;
    const [currentAppGroupData, setCurrentAppGroupData] = React.useState({});
    PageState.currentAppGroupData = currentAppGroupData;
    PageState.setCurrentAppGroupData = setCurrentAppGroupData;
    const [addFormDialogVisible, setAddFormDialogVisible] = React.useState(false);
    PageState.addFormDialogVisible = addFormDialogVisible;
    PageState.setAddFormDialogVisible = setAddFormDialogVisible;
    const [editFormDialogVisible, setEditFormDialogVisible] = React.useState(false);
    PageState.editFormDialogVisible = editFormDialogVisible;
    PageState.setEditFormDialogVisible = setEditFormDialogVisible;
    const [infoFormDialogVisible, setInfoFormDialogVisible] = React.useState(false);
    PageState.infoFormDialogVisible = infoFormDialogVisible;
    PageState.setInfoFormDialogVisible = setInfoFormDialogVisible;
    const [changeParentFormDialogVisible, setChangeParentFormDialogVisible] = React.useState(false);
    PageState.changeParentFormDialogVisible = changeParentFormDialogVisible;
    PageState.setChangeParentFormDialogVisible = setChangeParentFormDialogVisible;
    const [waitChangeAppGroupDataId, setWaitChangeAppGroupDataId] = React.useState("");
    PageState.waitChangeAppGroupDataId = waitChangeAppGroupDataId;
    PageState.setWaitChangeAppGroupDataId = setWaitChangeAppGroupDataId;
    const [parentTreeData, setParentTreeData] = React.useState([]);
    PageState.parentTreeData = parentTreeData;
    PageState.setParentTreeData = setParentTreeData;
    const [changeShowOrderFormDialogVisible, setChangeShowOrderFormDialogVisible] = React.useState(false);
    PageState.changeShowOrderFormDialogVisible = changeShowOrderFormDialogVisible;
    PageState.setChangeShowOrderFormDialogVisible = setChangeShowOrderFormDialogVisible;
    const [importFormDialogVisible, setImportFormDialogVisible] = React.useState(false);
    PageState.importFormDialogVisible = importFormDialogVisible;
    PageState.setImportFormDialogVisible = setImportFormDialogVisible;
    const [exportFormDialogVisible, setExportFormDialogVisible] = React.useState(false);
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
  findTreeData: (v) => {
    if (!v && v == "") {
      PageState.setSelectTreeNodeKeys([]);
      PageState.setSelectTreeNodeData({});
    }
    axios.post(PageUrl.findTreeUrl, { appGroupName: v }).then((resp) => {
      const { code, msg, data } = resp;
      if (code == 0) {
        if (data == null || data.length == 0) {
          PageState.setSelectTreeNodeData({});
          PageState.setSelectTreeNodeKeys([]);
        }
        PageState.setTreeData(data);
        PageAction.findGridData();
      } else {
        console.log("error:", resp);
        arco.Message.error(resp.msg);
      }
    }).catch((err) => {
      console.log("error:", err);
      arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
    });
  },
  findTreeDataIgnoreDataId: (dataId) => {
    axios.post(PageUrl.findIgnoreTreeUrl, { dataId }).then((resp) => {
      const { code, msg, data } = resp;
      if (code == 0) {
        PageState.setParentTreeData(data);
      } else {
        console.log("error:", resp);
        arco.Message.error(resp.msg);
      }
    }).catch((err) => {
      console.log("error:", err);
      arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
    });
  },
  findGridData: () => {
    let searchFormData = PageState.searchFormData;
    let pageParams = {
      pageNo: PageState.pageData.pageNo,
      pageSize: PageState.pageData.pageSize,
      params: {
        appGroupParentCode: PageState.selectTreeNodeData?.appGroupCode,
        ...searchFormData
      }
    };
    axios.post(PageUrl.findPageUrl, pageParams).then((resp) => {
      const { code, msg, data } = resp;
      if (code == 0) {
        PageState.setGridData(data.data);
        PageState.setPageData({ ...PageState.pageData, total: data.total });
      } else {
        console.log("error:", resp);
        arco.Message.error(resp.msg);
      }
    }).catch((err) => {
      console.log("error:", err);
      arco.Message.error("\u7CFB\u7EDF\u597D\u50CF\u662F\u8D70\u4E22\u4E86\uFF0C\u8BF7\u8054\u7CFB\u7BA1\u7406\u5458");
    });
  },
  addBrotherNode: (node) => {
    let appGroupParentCode = node.appGroupParentCode;
    let currentData = {
      appGroupParentCode
    };
    PageState.setCurrentAppGroupData(currentData);
    PageState.setAddFormDialogVisible(true);
  },
  addChildNode: (node) => {
    let appGroupParentCode = "#";
    if (node && node.appGroupCode) {
      appGroupParentCode = node.appGroupCode;
    } else {
      if (PageState.selectTreeNodeData && PageState.selectTreeNodeData.appGroupCode) {
        appGroupParentCode = PageState.selectTreeNodeData.appGroupCode;
      }
    }
    let currentData = {
      appGroupParentCode
    };
    PageState.setCurrentAppGroupData(currentData);
    PageState.setAddFormDialogVisible(true);
  },
  editNode: (node) => {
    let dataId = node.dataId;
    axios.post(PageUrl.findInfoUrl + "?dataId=" + dataId, {}).then((resp) => {
      if (resp.code == 0) {
        PageState.setCurrentAppGroupData(resp.data);
        PageState.setEditFormDialogVisible(true);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  save(appGroupData, callback) {
    axios.post(PageUrl.saveUrl, appGroupData).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        callback();
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  updateParent(appGroupData, callback) {
    axios.post(PageUrl.updateParentUrl, appGroupData).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        callback();
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  removeNode: (node) => {
    axios.post(PageUrl.removeUrl + "?dataId=" + node.dataId, {}).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        PageAction.findTreeData(null);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  batchRemoveNode: (dataIdList) => {
    axios.post(PageUrl.batchRemoveUrl, {
      dataIdList
    }).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        PageAction.findTreeData(null);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  enableNode: (node) => {
    axios.post(PageUrl.enableUrl + "?dataId=" + node.dataId, {}).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        PageAction.findTreeData(null);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  batchEnableNode: (dataIdList) => {
    axios.post(PageUrl.batchEnableUrl, {
      dataIdList
    }).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        PageAction.findTreeData(null);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  disableNode: (node) => {
    axios.post(PageUrl.disableUrl + "?dataId=" + node.dataId, {}).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        PageAction.findTreeData(null);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  batchDisableNode: (dataIdList) => {
    axios.post(PageUrl.batchDisableUrl, {
      dataIdList
    }).then((resp) => {
      if (resp.code == 0) {
        arco.Message.success(resp.msg);
        PageAction.findTreeData(null);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  },
  changeParentNode: (node) => {
    PageState.setWaitChangeAppGroupDataId(node.dataId);
    PageState.setChangeParentFormDialogVisible(true);
  },
  viewInfo(node) {
    let dataId = node.dataId;
    axios.post(PageUrl.findInfoUrl + "?dataId=" + dataId, {}).then((resp) => {
      if (resp.code == 0) {
        PageState.setCurrentAppGroupData(resp.data);
        PageState.setInfoFormDialogVisible(true);
      } else {
        arco.Message.error(resp.msg);
      }
    });
  }
};

// web/tsx/app_group/form/form.tsx
var AppGroupForm = () => {
  React.useEffect(() => {
    if (PageState.currentAppGroupData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentAppGroupData);
    }
  }, [PageState.currentAppGroupData]);
  return /* @__PURE__ */ React.createElement(arco.Form, {
    ref: PageState.addFormRef
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u4E3B\u952E",
    field: "dataId",
    hidden: true
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u4E0A\u7EA7\u5206\u7EC4\u7F16\u7801",
    field: "appGroupParentCode",
    hidden: true
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u5206\u7EC4\u7F16\u7801",
    field: "appGroupCode",
    rules: [
      { required: true, message: "\u8BF7\u8F93\u5165\u5206\u7EC4\u7F16\u7801" },
      { maxLength: 32, message: "\u6700\u591A\u8F93\u5165 32 \u4E2A\u5B57" }
    ]
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u5206\u7EC4\u7F16\u7801"
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u5206\u7EC4\u540D\u79F0",
    field: "appGroupName",
    rules: [
      { required: true, message: "\u8BF7\u8F93\u5165\u5206\u7EC4\u540D\u79F0" },
      { maxLength: 32, message: "\u6700\u591A\u8F93\u5165 32 \u4E2A\u5B57" }
    ]
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u5206\u7EC4\u540D\u79F0"
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u5206\u7EC4\u63CF\u8FF0",
    field: "appGroupDesc",
    rules: [{ maxLength: 256, message: "\u6700\u591A\u8F93\u5165 256 \u4E2A\u5B57\u7B26" }]
  }, /* @__PURE__ */ React.createElement(arco.Input.TextArea, {
    placeholder: "\u8BF7\u8F93\u5165\u5206\u7EC4\u63CF\u8FF0"
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u663E\u793A\u987A\u5E8F",
    field: "appGroupOrder"
  }, /* @__PURE__ */ React.createElement(arco.InputNumber, {
    placeholder: "\u8BF7\u8F93\u5165\u987A\u5E8F"
  })));
};
var AppGroupInfoForm = () => {
  React.useEffect(() => {
    if (PageState.currentAppGroupData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentAppGroupData);
    }
  }, [PageState.currentAppGroupData]);
  return /* @__PURE__ */ React.createElement(arco.Form, {
    ref: PageState.addFormRef
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u4E3B\u952E",
    field: "dataId",
    hidden: true
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u4E0A\u7EA7\u5206\u7EC4\u7F16\u7801",
    field: "appGroupParentCode",
    hidden: true
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u5206\u7EC4\u7F16\u7801",
    field: "appGroupCode"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "",
    readOnly: true
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u5206\u7EC4\u540D\u79F0",
    field: "appGroupName"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "",
    readOnly: true
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u5206\u7EC4\u63CF\u8FF0",
    field: "appGroupDesc"
  }, /* @__PURE__ */ React.createElement(arco.Input.TextArea, {
    placeholder: "",
    readOnly: true
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u663E\u793A\u987A\u5E8F",
    field: "appGroupOrder"
  }, /* @__PURE__ */ React.createElement(arco.InputNumber, {
    placeholder: "",
    readOnly: true
  })));
};
var AppGroupParentForm = () => {
  React.useEffect(() => {
    if (PageState.waitChangeAppGroupDataId && PageState.changeParentFormDialogVisible) {
      PageAction.findTreeDataIgnoreDataId(PageState.waitChangeAppGroupDataId);
      PageState.changeParentFormRef?.current.setFieldsValue({
        dataId: PageState.waitChangeAppGroupDataId
      });
    }
  }, [PageState.waitChangeAppGroupDataId]);
  return /* @__PURE__ */ React.createElement(arco.Form, {
    ref: PageState.changeParentFormRef
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u4E3B\u952E",
    field: "dataId",
    hidden: true
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: ""
  })), /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u76EE\u6807\u5206\u7EC4",
    field: "appGroupParentCode",
    hidden: false
  }, /* @__PURE__ */ React.createElement(arco.TreeSelect, {
    treeData: PageState.parentTreeData,
    fieldNames: {
      key: "appGroupCode",
      title: "appGroupName",
      children: "appGroupChildren"
    },
    placeholder: "\u8BF7\u9009\u62E9\u76EE\u6807\u5206\u7EC4"
  })));
};

// web/tsx/app_group/form/dialog.tsx
var AddAppGroupFormDialog = () => {
  return /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "\u65B0\u589E\u7EC4\u7EC7",
    visible: PageState.addFormDialogVisible,
    onOk: () => {
      PageState.addFormRef.current?.validate().then((data) => {
        PageAction.save(data, () => {
          PageState.setCurrentAppGroupData(null);
          PageState.setAddFormDialogVisible(false);
          PageState.addFormRef.current?.resetFields();
          PageAction.findTreeData(null);
        });
      });
    },
    onCancel: () => {
      PageState.addFormRef.current?.resetFields();
      PageState.setAddFormDialogVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(AppGroupForm, null));
};
var EditAppGroupFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "\u7F16\u8F91\u7EC4\u7EC7",
    visible: PageState.editFormDialogVisible,
    onOk: () => {
      PageState.addFormRef.current?.validate().then((data) => {
        PageAction.save(data, () => {
          PageState.setCurrentAppGroupData(null);
          PageState.setEditFormDialogVisible(false);
          PageState.addFormRef.current?.resetFields();
          PageAction.findTreeData(null);
        });
      });
    },
    onCancel: () => {
      PageState.editFormRef.current?.resetFields();
      PageState.setEditFormDialogVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(AppGroupForm, null)));
};
var InfoAppGroupFormDialog = () => {
  return /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "\u67E5\u770B\u7EC4\u7EC7",
    visible: PageState.infoFormDialogVisible,
    onCancel: () => PageState.setInfoFormDialogVisible(false),
    footer: null
  }, /* @__PURE__ */ React.createElement(AppGroupInfoForm, null));
};
var ChangeParentAppGroupFormDialog = () => {
  return /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "\u53D8\u66F4\u4E0A\u7EA7",
    visible: PageState.changeParentFormDialogVisible,
    onOk: () => {
      PageState.changeParentFormRef.current?.validate().then((data) => {
        debugger;
        PageAction.updateParent(data, () => {
          PageState.setChangeParentFormDialogVisible(false);
          PageState.changeParentFormRef.current?.resetFields();
          PageAction.findTreeData("");
        });
      });
    },
    onCancel: () => {
      PageState.changeParentFormRef.current?.resetFields();
      PageState.setChangeParentFormDialogVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(AppGroupParentForm, null));
};
var ImportAppGroupFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "\u5BFC\u5165\u7EC4\u7EC7",
    visible: PageState.importFormDialogVisible,
    onOk: () => PageState.setImportFormDialogVisible(false),
    onCancel: () => PageState.setImportFormDialogVisible(false)
  }));
};
var ExportAppGroupFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "\u5BFC\u51FA\u7EC4\u7EC7",
    visible: PageState.exportFormDialogVisible,
    onOk: () => PageState.setExportFormDialogVisible(false),
    onCancel: () => PageState.setExportFormDialogVisible(false)
  }));
};

// web/tsx/app_group/grid.tsx
var PageGridView = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-container"
  }, /* @__PURE__ */ React.createElement(PageGridSearchForm, null), /* @__PURE__ */ React.createElement(PageGridToolBar, null), /* @__PURE__ */ React.createElement(PageGridTable, null), /* @__PURE__ */ React.createElement(PageGridPage, null));
};
var PageGridSearchForm = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-search"
  }, /* @__PURE__ */ React.createElement(arco.Form, {
    colon: true,
    ref: PageState.searchFormRef
  }, /* @__PURE__ */ React.createElement(arco.Grid.Row, {
    guides: [1, 1]
  }, /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 7
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u7F16\u7801",
    field: "appGroupCode"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u5206\u7EC4\u7F16\u7801",
    allowClear: true
  }))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 7
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u540D\u79F0",
    field: "appGroupName"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165\u5206\u7EC4\u540D\u79F0",
    allowClear: true
  }))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 7
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "\u72B6\u6001",
    field: "dataStatus"
  }, /* @__PURE__ */ React.createElement(arco.Select, {
    placeholder: "\u8BF7\u9009\u62E9\u72B6\u6001",
    allowClear: true
  }, /* @__PURE__ */ React.createElement(arco.Select.Option, {
    key: "1",
    value: "1"
  }, "\u5DF2\u542F\u7528"), /* @__PURE__ */ React.createElement(arco.Select.Option, {
    key: "0",
    value: "0"
  }, "\u5DF2\u505C\u7528")))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 3
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, null, /* @__PURE__ */ React.createElement(arco.Space, null, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    style: { marginLeft: "8px" },
    onClick: () => {
      let formData = PageState.searchFormRef.current.getFieldsValue();
      PageState.setSearchFormData(formData);
    }
  }, "\u67E5\u8BE2"), /* @__PURE__ */ React.createElement(arco.Button, {
    onClick: () => {
      PageState.searchFormRef.current.resetFields();
    }
  }, "\u6E05\u7A7A")))))), /* @__PURE__ */ React.createElement(arco.Divider, {
    style: { margin: "0px 0 4px 0 " }
  }));
};
var PageGridToolBar = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-toolbar"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-toolbar major"
  }, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    onClick: () => {
      PageAction.addChildNode(PageState.selectTreeNodeData?.dataRef);
    }
  }, "\u65B0\u589E"), PageState.selectedRowKeys && PageState.selectedRowKeys.length > 0 ? /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "secondary",
    onClick: () => {
      PageAction.batchEnableNode(PageState.selectedRowKeys);
    }
  }, "\u542F\u7528"), /* @__PURE__ */ React.createElement(arco.Button, {
    type: "secondary",
    onClick: () => {
      PageAction.batchDisableNode(PageState.selectedRowKeys);
    }
  }, "\u505C\u7528"), /* @__PURE__ */ React.createElement(arco.Popconfirm, {
    focusLock: true,
    title: "\u5220\u9664\u786E\u8BA4",
    content: "\u8BE5\u6570\u636E\u53CA\u5B50\u6570\u636E\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\uFF0C\u786E\u5B9A\u5220\u9664\u5417?",
    onOk: () => {
      PageAction.batchRemoveNode(PageState.selectedRowKeys);
    },
    onCancel: () => {
    }
  }, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    status: "danger"
  }, "\u5220\u9664"))) : null));
};
var PageGridTable = () => {
  const enableAction = (record) => {
    return [
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u65B0\u589E\u5B50\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconPlus, null),
        size: "mini",
        onClick: () => {
          PageAction.addChildNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u67E5\u770B"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconEye, null),
        size: "mini",
        onClick: () => {
          PageAction.viewInfo(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u505C\u7528"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconStop, null),
        size: "mini",
        onClick: () => {
          PageAction.disableNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u53D8\u66F4\u4E0A\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconStrikethrough, null),
        size: "mini",
        onClick: () => {
          PageAction.changeParentNode(record);
        }
      }))
    ];
  };
  const disableAction = (record) => {
    return [
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u65B0\u589E\u5B50\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconPlus, null),
        size: "mini",
        onClick: () => {
          PageAction.addChildNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u7F16\u8F91"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconEdit, null),
        size: "mini",
        onClick: () => {
          PageAction.editNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u542F\u7528"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconPlayArrow, null),
        size: "mini",
        onClick: () => {
          PageAction.enableNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u53D8\u66F4\u4E0A\u7EA7"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "secondary",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconStrikethrough, null),
        size: "mini",
        onClick: () => {
          PageAction.changeParentNode(record);
        }
      })),
      /* @__PURE__ */ React.createElement(arco.Popconfirm, {
        focusLock: true,
        title: "\u5220\u9664\u786E\u8BA4",
        content: "\u8BE5\u6570\u636E\u53CA\u5B50\u6570\u636E\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\uFF0C\u786E\u5B9A\u5220\u9664\u5417?",
        onOk: () => {
          PageAction.removeNode(record);
        },
        onCancel: () => {
        }
      }, /* @__PURE__ */ React.createElement(arco.Tooltip, {
        content: "\u5220\u9664"
      }, /* @__PURE__ */ React.createElement(arco.Button, {
        type: "primary",
        status: "danger",
        icon: /* @__PURE__ */ React.createElement(arcoicon.IconDelete, null),
        size: "mini"
      })))
    ];
  };
  const gridColumn = [
    {
      width: 200,
      ellipsis: true,
      title: "\u7F16\u7801",
      dataIndex: "appGroupCode"
    },
    {
      width: 300,
      ellipsis: true,
      title: "\u540D\u79F0",
      dataIndex: "appGroupName"
    },
    {
      ellipsis: true,
      title: "\u4E0A\u7EA7\u5206\u7EC4",
      dataIndex: "appGroupNamePath",
      render: (col, record, index) => {
        const path = record.appGroupNamePath;
        const parentPath = path.replace("/" + record.appGroupName + "/", "");
        return parentPath;
      }
    },
    {
      width: 100,
      ellipsis: true,
      title: "\u663E\u793A\u987A\u5E8F",
      dataIndex: "appGroupOrder"
    },
    {
      title: "\u72B6\u6001",
      dataIndex: "dataStatus",
      fixed: "right",
      width: 80,
      render: (value) => {
        if (value === "1") {
          return /* @__PURE__ */ React.createElement(arco.Tag, {
            color: "green"
          }, "\u5DF2\u542F\u7528");
        } else {
          return /* @__PURE__ */ React.createElement(arco.Tag, {
            color: "red"
          }, "\u5DF2\u505C\u7528");
        }
      }
    },
    {
      title: "\u64CD\u4F5C",
      dataIndex: "action",
      width: 180,
      fixed: "right",
      align: "left",
      render: (value, record, index) => {
        return /* @__PURE__ */ React.createElement(arco.Space, null, record.dataStatus === "1" ? enableAction(record) : disableAction(record));
      }
    }
  ];
  const gridRowSelection = {
    checkAll: true,
    checkCrossPage: true,
    fixed: true,
    columnWidth: 40,
    type: "checkbox",
    selectedRowKeys: PageState.selectedRowKeys,
    onChange: (selectedRowKeys, selectedRows) => {
      PageState.setSelectedRowKeys(selectedRowKeys);
      PageState.setSelectedRows(selectedRows);
    },
    onSelect: (selected, record, selectedRows) => {
    }
  };
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-table"
  }, /* @__PURE__ */ React.createElement(arco.Table, {
    rowKey: "dataId",
    columns: gridColumn,
    data: PageState.gridData,
    rowSelection: gridRowSelection,
    pagination: false,
    stripe: true,
    border: {
      wrapper: true,
      cell: true
    }
  }));
};
var PageGridPage = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-page"
  }, /* @__PURE__ */ React.createElement(arco.Pagination, {
    size: PageState.showPageSize,
    total: PageState.pageData.total,
    showTotal: true,
    showJumper: true,
    sizeCanChange: true,
    onChange: (pageNo, pageSize) => {
      PageState.setPageData({
        ...PageState.pageData,
        pageNo,
        pageSize
      });
    }
  }));
};

// web/tsx/app_group/tree.tsx
var PageTreeView = () => {
  const TreeNodExtraAction = (props) => {
    const data = props.dataRef;
    return /* @__PURE__ */ React.createElement(arco.Menu, {
      style: {
        width: "100px",
        background: "#fff",
        border: "1px solid #e8e8e8"
      }
    }, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addBrother",
      onClick: () => {
        PageAction.addBrotherNode(data);
      }
    }, "\u65B0\u589E\u540C\u7EA7"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addChild",
      onClick: () => {
        PageAction.addChildNode(data);
      }
    }, "\u65B0\u589E\u5B50\u7EA7"), data.dataStatus === "0" ? /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "edit",
      onClick: () => {
        PageAction.editNode(data);
      }
    }, "\u7F16\u8F91", " ") : null, data.dataStatus === "0" ? /* @__PURE__ */ React.createElement(arco.Popconfirm, {
      focusLock: true,
      title: "\u5220\u9664\u786E\u8BA4",
      content: "\u8BE5\u6570\u636E\u53CA\u5B50\u6570\u636E\u5220\u9664\u540E\u65E0\u6CD5\u6062\u590D\uFF0C\u786E\u5B9A\u5220\u9664\u5417?",
      onOk: () => {
        PageAction.removeNode(data);
      },
      onCancel: () => {
      }
    }, /* @__PURE__ */ React.createElement(arco.Button, {
      type: "primary"
    }, "\u5220\u9664")) : null, data.dataStatus === "0" ? /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "enable",
      onClick: () => {
        PageAction.enableNode(data);
      }
    }, "\u542F\u7528") : null, data.dataStatus === "1" ? /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "disable",
      onClick: () => {
        PageAction.disableNode(data);
      }
    }, "\u505C\u7528") : null, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "changeParent",
      onClick: () => {
        PageAction.changeParentNode(data);
      }
    }, "\u53D8\u66F4\u7236\u7EA7"));
  };
  const renderTreeNodExtra = (node) => {
    return /* @__PURE__ */ React.createElement(arco.Dropdown, {
      droplist: /* @__PURE__ */ React.createElement(TreeNodExtraAction, {
        dataRef: node.dataRef
      }),
      position: "bl"
    }, /* @__PURE__ */ React.createElement(arcoicon.IconMore, {
      style: {
        position: "absolute",
        right: 8,
        fontSize: 12,
        top: 10,
        color: "#3370ff"
      }
    }));
  };
  return /* @__PURE__ */ React.createElement("div", null, /* @__PURE__ */ React.createElement("div", {
    style: { display: "block" }
  }, /* @__PURE__ */ React.createElement(arco.Input.Search, {
    searchButton: true,
    placeholder: "\u8BF7\u8F93\u5165",
    onSearch: (v) => {
      PageAction.findTreeData(v);
    }
  })), /* @__PURE__ */ React.createElement(arco.Tree, {
    treeData: PageState.treeData,
    blockNode: true,
    renderExtra: renderTreeNodExtra,
    onSelect: (keys, extra) => {
      PageState.setSelectTreeNodeKeys(keys);
      PageState.setSelectTreeNodeData(extra.node.props.dataRef);
    },
    showLine: true,
    selectedKeys: PageState.selectTreeNodeKeys,
    fieldNames: {
      key: "appGroupCode",
      title: "appGroupName",
      children: "appGroupChildren"
    }
  }));
};

// web/tsx/app_group/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(PageView, null));
};
var PageView = (props) => {
  PageAction.init(props);
  React.useEffect(() => {
    PageAction.findTreeData(null);
  }, []);
  React.useEffect(() => {
    PageAction.findGridData();
  }, [
    PageState.pageData.pageNo,
    PageState.pageData.pageSize,
    PageState.selectTreeNodeData,
    PageState.searchFormData
  ]);
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-app-fluid"
  }, /* @__PURE__ */ React.createElement(arco.Grid.Row, {
    guides: [1, 1],
    style: { height: "100vh" }
  }, /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    flex: "260px"
  }, /* @__PURE__ */ React.createElement(PageTreeView, null)), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical",
    style: { height: "100%" }
  }), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    flex: "auto",
    style: { height: "100%", width: "600px" }
  }, /* @__PURE__ */ React.createElement(PageGridView, null))), /* @__PURE__ */ React.createElement(AddAppGroupFormDialog, null), /* @__PURE__ */ React.createElement(EditAppGroupFormDialog, null), /* @__PURE__ */ React.createElement(InfoAppGroupFormDialog, null), /* @__PURE__ */ React.createElement(ChangeParentAppGroupFormDialog, null), /* @__PURE__ */ React.createElement(ImportAppGroupFormDialog, null), /* @__PURE__ */ React.createElement(ExportAppGroupFormDialog, null));
};
