// web/tsx/app_group/action.tsx
var PageState = {};
var PageAction = {
  init: () => {
    const [treeSearchValue, setTreeSearchValue] = React.useState("");
    PageState.treeSearchValue = treeSearchValue;
    PageState.setTreeSearchValue = setTreeSearchValue;
    const [treeData, setTreeData] = React.useState([]);
    PageState.treeData = treeData;
    PageState.setTreeData = setTreeData;
    const [treeSelectedKeys, setTreeSelectedKeys] = React.useState([]);
    PageState.treeSelectedKeys = treeSelectedKeys;
    PageState.setTreeSelectedKeys = setTreeSelectedKeys;
    const [treeExpandedKeys, setTreeExpandedKeys] = React.useState([]);
    PageState.treeExpandedKeys = treeExpandedKeys;
    PageState.setTreeExpandedKeys = setTreeExpandedKeys;
    const [treeSelectedNodeData, setTreeSelectedNodeData] = React.useState([]);
    PageState.treeSelectedNodeData = treeSelectedNodeData;
    PageState.setTreeSelectedNodeData = setTreeSelectedNodeData;
    const [searchFormData, setSearchFormData] = React.useState({});
    PageState.searchFormData = searchFormData;
    PageState.setSearchFormData = setSearchFormData;
    PageState.searchFormRef = React.useRef();
    const [tableData, setTableData] = React.useState([]);
    PageState.tableData = tableData;
    PageState.setTableData = setTableData;
    const [pageData, setPageData] = React.useState({
      pageNum: 1,
      pageSize: 10,
      total: 0
    });
    PageState.pageData = pageData;
    PageState.setPageData = setPageData;
    const [tableSelectedKeys, setTableSelectedKeys] = React.useState([]);
    PageState.tableSelectedKeys = tableSelectedKeys;
    PageState.setTableSelectedKeys = setTableSelectedKeys;
    const [tableSelectedNode, setTableSelectedNode] = React.useState([]);
    PageState.tableSelectedNode = tableSelectedNode;
    PageState.setTableSelectedNode = setTableSelectedNode;
    const [formData, setFormData] = React.useState({});
    PageState.formData = formData;
    PageState.setFormData = setFormData;
    const [addFormVisible, setAddFormVisible] = React.useState(false);
    PageState.addFormVisible = addFormVisible;
    PageState.setAddFormVisible = setAddFormVisible;
    PageState.addFormRef = React.useRef();
    const [editFormVisible, setEditFormVisible] = React.useState(false);
    PageState.editFormVisible = editFormVisible;
    PageState.setEditFormVisible = setEditFormVisible;
    PageState.editFormRef = React.useRef();
    const [detailFormVisible, setDetailFormVisible] = React.useState(false);
    PageState.detailFormVisible = detailFormVisible;
    PageState.setDetailFormVisible = setDetailFormVisible;
    PageState.detailFormRef = React.useRef();
    const [changeOwnerFormVisible, setChangeOwnerFormVisible] = React.useState(false);
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
  saveData: (data, callback) => {
  },
  removeData: (data) => {
  },
  enableData: (data) => {
  },
  disableData: (data) => {
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
  }
};

// web/tsx/app_group/form.tsx
var AddForm = () => {
  return /* @__PURE__ */ React.createElement("div", null, "AddForm");
};
var EditForm = () => {
  return /* @__PURE__ */ React.createElement("div", null, "EditForm");
};
var InfoForm = () => {
  return /* @__PURE__ */ React.createElement("div", null, "EditForm");
};
var OwnerForm = () => {
  return /* @__PURE__ */ React.createElement("div", null, "EditForm");
};

// web/tsx/app_group/dialog.tsx
var AddFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "新增分组",
    style: { width: "800px" },
    visible: PageState.addFormVisible,
    onOk: () => {
      PageState.addFormRef.current?.validate().then((data) => {
        PageAction.saveData(data, () => {
          PageAction.findTreeData();
          PageState.setFormData(null);
          PageState.addFormRef.current?.resetFields();
          PageState.setAddFormVisible(false);
        });
      });
    },
    onCancel: () => {
      PageState.setFormData(null);
      PageState.addFormRef.current?.resetFields();
      PageState.setAddFormVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(AddForm, null)));
};
var EditFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "编辑分组",
    style: { width: "800px" },
    visible: PageState.editFormVisible,
    onOk: () => {
      PageState.editFormRef.current?.validate().then((data) => {
        PageAction.saveData(data, () => {
          PageAction.findTreeData();
          PageState.setFormData(null);
          PageState.editFormRef.current?.resetFields();
          PageState.setEditFormVisible(false);
        });
      });
    },
    onCancel: () => {
      PageState.setFormData(null);
      PageState.editFormRef.current?.resetFields();
      PageState.setEditFormVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(EditForm, null)));
};
var InfoFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "查看分组",
    style: { width: "800px" },
    visible: PageState.detailFormVisible,
    footer: null,
    onCancel: () => {
      PageState.setFormData(null);
      PageState.detailFormRef.current?.resetFields();
      PageState.setDetailFormVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(InfoForm, null)));
};
var OwnerFormDialog = () => {
  return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Modal, {
    title: "变更所属分组",
    style: { width: "800px" },
    visible: PageState.changeOwnerFormVisible,
    onOk: () => {
      PageState.changeOwnerFormRef.current?.validate().then((data) => {
        PageAction.saveData(data, () => {
          PageAction.findTreeData();
          PageState.setFormData(null);
          PageState.changeOwnerFormRef.current?.resetFields();
          PageState.setChangeOwnerFormVisible(false);
        });
      });
    },
    onCancel: () => {
      PageState.setFormData(null);
      PageState.changeOwnerFormRef.current?.resetFields();
      PageState.setChangeOwnerFormVisible(false);
    }
  }, /* @__PURE__ */ React.createElement(OwnerForm, null)));
};

// web/tsx/app_group/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(PageView, null));
};
var PageView = () => {
  PageAction.init();
  React.useEffect(() => {
    PageAction.findTreeData(null);
  }, []);
  React.useEffect(() => {
    PageAction.findPageData();
  }, [
    PageState.pageData.pageNum,
    PageState.pageData.pageSize,
    PageState.treeSelectedNodeData,
    PageState.searchFormData
  ]);
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_container"
  }, /* @__PURE__ */ React.createElement(TreeGridPageView, null));
};
var TreeGridPageView = (props) => {
  React.useEffect(() => {
    const treeData = [
      {
        title: "Trunk 0-0",
        key: "0-0",
        children: [
          {
            title: "Branch 0-0-2",
            key: "0-0-2",
            selectable: false,
            children: [
              {
                title: "Leaf",
                key: "0-0-2-1",
                children: [
                  {
                    title: "Leafsss 0-0-2",
                    key: "0-0-2-1-0",
                    children: [
                      {
                        title: "Leaf",
                        key: "0-0-2-1-0-0"
                      }
                    ]
                  }
                ]
              }
            ]
          }
        ]
      },
      {
        title: "Trunk 0-1",
        key: "0-1",
        children: [
          {
            title: "Branch 0-1-1",
            key: "0-1-1",
            children: [
              {
                title: "Leaf",
                key: "0-1-1-0"
              }
            ]
          }
        ]
      }
    ];
    PageState.setTreeData(treeData);
  }, []);
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_page_tree_grid"
  }, /* @__PURE__ */ React.createElement(TreePanel, null), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical",
    style: { width: "1px", height: "100%", margin: "0 2px" }
  }), /* @__PURE__ */ React.createElement(GridPanel, null), /* @__PURE__ */ React.createElement(AddFormDialog, null), /* @__PURE__ */ React.createElement(EditFormDialog, null), /* @__PURE__ */ React.createElement(InfoFormDialog, null), /* @__PURE__ */ React.createElement(OwnerFormDialog, null));
};
var TreeHeader = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "tree_header"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "tree_title"
  }, "应用分组"), /* @__PURE__ */ React.createElement("div", {
    className: "tree_action"
  }, /* @__PURE__ */ React.createElement(BmbpIconFont, {
    type: "icon-zhongzhi",
    onClick: () => {
      PageState.setTreeSelectedNodeData(null);
      PageState.setTreeSearchValue("");
      PageAction.findTreeData();
    }
  }), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical"
  }), /* @__PURE__ */ React.createElement(BmbpIconFont, {
    type: "icon-jia1",
    onClick: () => {
      PageAction.openAddBrotherForm(null);
    }
  })));
};
var TreeSearchBody = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "tree_search"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "请输入应用分组名称",
    style: { width: "100%" },
    onChange: (v) => {
      PageState.setTreeSearchValue(v);
    }
  }));
};
var TreePanelBody = () => {
  const generatorTreeNodes = (treeData) => {
    return treeData.map((item) => {
      const { children, key, ...rest } = item;
      return /* @__PURE__ */ React.createElement(arco.Tree.Node, {
        key,
        ...rest,
        dataRef: item
      }, children ? generatorTreeNodes(item.children) : null);
    });
  };
  const TreeNodeAction = (props) => {
    const nodeData = props.node.dataRef;
    const dropList = /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Menu, {
      className: "tree_node_action_menu"
    }, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addBrotherNode",
      className: "menu_item",
      onClick: () => {
        PageAction.openAddBrotherForm(nodeData);
      }
    }, "新增兄弟节点"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addChildrenNode",
      className: "menu_item",
      onClick: () => {
        PageAction.openAddChildForm(nodeData);
      }
    }, "新增子节点"), nodeData.dataStatus === "0" ? /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "editNode",
      className: "menu_item",
      onClick: () => {
        PageAction.openEditForm(nodeData);
      }
    }, "编辑"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "changeParent",
      className: "menu_item",
      onClick: () => {
        PageAction.openChangeParentForm(nodeData);
      }
    }, "变更上级"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "enableNode",
      className: "menu_item",
      onClick: () => {
        PageAction.enableData(nodeData);
      }
    }, "启用"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "deleteNode",
      className: "menu_item"
    }, /* @__PURE__ */ React.createElement(arco.Popconfirm, {
      focusLock: true,
      title: "删除确认",
      content: "删除后数据将不再存在，是否删除?",
      onOk: () => {
        PageAction.removeData(record);
      },
      onCancel: () => {
      }
    }, "删除"))) : /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "disableNode",
      className: "menu_item",
      onClick: () => {
        PageAction.disableData(nodeData);
      }
    }, "停用")), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "infoNode",
      className: "menu_item",
      onClick: () => {
        PageAction.openInfoForm(nodeData);
      }
    }, "查看")));
    return /* @__PURE__ */ React.createElement("div", {
      className: "tree_node_action"
    }, /* @__PURE__ */ React.createElement(arco.Dropdown, {
      droplist: dropList,
      position: "br"
    }, /* @__PURE__ */ React.createElement(BmbpIconFont, {
      type: "icon-zuoyouduiqi"
    })));
  };
  return /* @__PURE__ */ React.createElement("div", {
    className: "tree_body"
  }, /* @__PURE__ */ React.createElement(arco.Tree, {
    blockNode: true,
    renderExtra: (node) => {
      return /* @__PURE__ */ React.createElement(TreeNodeAction, {
        node
      });
    }
  }, generatorTreeNodes(PageState.treeData)));
};
var TreePanel = (props) => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_page_tree",
    style: { width: "300px" }
  }, /* @__PURE__ */ React.createElement(TreeHeader, null), /* @__PURE__ */ React.createElement(TreeSearchBody, null), /* @__PURE__ */ React.createElement(TreePanelBody, null));
};
var GridPanel = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_page_grid"
  }, /* @__PURE__ */ React.createElement(SearchForm, null), /* @__PURE__ */ React.createElement(arco.Divider, {
    style: { margin: "4px 0 4px 0 " }
  }), /* @__PURE__ */ React.createElement(GridToolBar, null), /* @__PURE__ */ React.createElement(GridTable, null), /* @__PURE__ */ React.createElement(GridPage, null));
};
var SearchForm = (props) => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "search_form"
  }, /* @__PURE__ */ React.createElement(SearchFormBody, null), /* @__PURE__ */ React.createElement(SearchFormAction, null));
};
var SearchFormBody = (props) => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "search_body"
  }, /* @__PURE__ */ React.createElement(arco.Form, {
    ref: props.searchFormRef
  }, /* @__PURE__ */ React.createElement(arco.Grid.Row, null, /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 12
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "分组名称",
    field: "node_name"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "请输入分组名称"
  }))), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    span: 12
  }, /* @__PURE__ */ React.createElement(arco.Form.Item, {
    label: "分组状态",
    field: "data_status"
  }, /* @__PURE__ */ React.createElement(arco.Select, {
    placeholder: "请选择分组状态"
  }, /* @__PURE__ */ React.createElement(arco.Option, {
    value: "0",
    key: "0"
  }, "已停用"), /* @__PURE__ */ React.createElement(arco.Option, {
    value: "1",
    key: "1"
  }, "已启用")))))));
};
var SearchFormAction = (props) => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "search_action"
  }, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    onClick: () => {
      const searchFormData = PageState.searchFormRef.current?.getFieldsValue();
      PageState.setSearchFormData(searchFormData);
    }
  }, "查询"), /* @__PURE__ */ React.createElement(arco.Button, {
    onClick: () => {
      PageState.searchFormRef.current?.resetFields();
      PageState.setSearchFormData({});
    }
  }, "重置"));
};
var GridToolBar = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "grid_tool_bar"
  }, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary",
    onClick: () => {
      PageAction.openAddChildForm(PageState.treeSelectNodeData?.dataRef);
    }
  }, "新增"));
};
var GridTable = () => {
  const RowDisableAction = (record2) => {
    return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Button, {
      type: "text",
      onClick: () => {
        PageAction.openEditForm(record2);
      }
    }, "编辑"), /* @__PURE__ */ React.createElement(arco.Button, {
      type: "text",
      onClick: () => {
        PageAction.enableData(record2);
      }
    }, "启用"), /* @__PURE__ */ React.createElement(arco.Popconfirm, {
      focusLock: true,
      title: "删除确认",
      content: "删除后数据将不再存在，是否删除?",
      onOk: () => {
        PageAction.removeData(record2);
      },
      onCancel: () => {
      }
    }, /* @__PURE__ */ React.createElement(arco.Button, {
      type: "text",
      status: "danger"
    }, "删除")));
  };
  const RowEnableAction = (record2) => {
    return /* @__PURE__ */ React.createElement(React.Fragment, null, /* @__PURE__ */ React.createElement(arco.Button, {
      type: "text",
      onClick: () => {
        PageAction.disableData(record2);
      }
    }, "停用"));
  };
  const tableColumnConfig = [
    {
      title: "序号",
      dataIndex: "",
      render: (_, record2, index) => {
        return PageState.pageData.pageSize * (PageState.pageData.pageNum - 1) + index + 1;
      },
      width: 80
    },
    {
      title: "分组名称",
      dataIndex: "node_name",
      width: 150
    },
    {
      title: "分组标签",
      dataIndex: "group_tag"
    },
    {
      title: "分组状态",
      dataIndex: "data_status",
      width: 100,
      render: (value, record2, index) => {
        return /* @__PURE__ */ React.createElement(arco.Tag, {
          color: value === "1" ? "green" : "red"
        }, value === "1" ? "已启用" : "已停用");
      }
    },
    {
      title: "操作",
      width: 200,
      render: (value, record2, index) => {
        return /* @__PURE__ */ React.createElement("div", {
          className: "grid_row_action"
        }, record2.data_status == "1" ? /* @__PURE__ */ React.createElement(RowEnableAction, {
          data: record2
        }) : /* @__PURE__ */ React.createElement(RowDisableAction, {
          data: record2
        }), /* @__PURE__ */ React.createElement(arco.Button, {
          type: "text",
          style: { color: "#666" },
          onClick: () => {
            PageAction.openInfoForm(record2);
          }
        }, "查看"));
      }
    }
  ];
  return /* @__PURE__ */ React.createElement("div", {
    className: "grid_table"
  }, /* @__PURE__ */ React.createElement(arco.Table, {
    stripe: true,
    columns: tableColumnConfig,
    data: PageState.tableData,
    pagination: false
  }));
};
var GridPage = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "grid_page"
  }, /* @__PURE__ */ React.createElement(arco.Pagination, {
    total: PageState.pageData.total,
    showTotal: true,
    showJumper: true,
    sizeCanChange: true,
    onChange: (pageNum, pageSize) => {
      PageState.setPageData({ ...PageState.pageData, pageNum, pageSize });
    }
  }));
};
