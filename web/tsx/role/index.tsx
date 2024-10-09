import { PageAction, PageState } from "./action";
import {
  AddRoleFormDialog,
  ChangeParentRoleFormDialog,
  EditRoleFormDialog,
  ExportRoleFormDialog,
  ImportRoleFormDialog,
  InfoRoleFormDialog,
} from "./form/dialog";

window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(<PageView />);
};
const PageView = (props) => {
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
    PageState.searchFormData,
  ]);
  return (
    <div className="bmbp-app-fluid">
      <arco.Grid.Row guides={[1, 1]} style={{ height: "100vh" }}>
        <arco.Grid.Col flex={"260px"}>
          <PageTreeView />
        </arco.Grid.Col>
        <arco.Divider type="vertical" style={{ height: "100%" }}></arco.Divider>
        <arco.Grid.Col flex={"auto"} style={{ height: "100%", width: "600px" }}>
          <PageGridView />
        </arco.Grid.Col>
      </arco.Grid.Row>
      <AddRoleFormDialog />
      <EditRoleFormDialog />
      <InfoRoleFormDialog />
      <ChangeParentRoleFormDialog />
      <ImportRoleFormDialog />
      <ExportRoleFormDialog />
    </div>
  );
};
const PageTreeView = () => {
  const TreeNodExtraAction = (props) => {
    const data = props.dataRef;
    debugger;
    return (
      <arco.Menu
        style={{
          width: "100px",
          background: "#fff",
          border: "1px solid #e8e8e8",
        }}
      >
        <arco.Menu.Item
          key="addBrother"
          onClick={() => {
            PageAction.addBrotherNode(data);
          }}
        >
          新增同级
        </arco.Menu.Item>
        <arco.Menu.Item
          key="addChild"
          onClick={() => {
            PageAction.addChildNode(data);
          }}
        >
          新增子级
        </arco.Menu.Item>
        {data.dataStatus === "0" ? (
          <arco.Menu.Item
            key="edit"
            onClick={() => {
              PageAction.editNode(data);
            }}
          >
            编辑{" "}
          </arco.Menu.Item>
        ) : null}
        {data.dataStatus === "0" ? (
          <arco.Menu.Item
            key="remove"
            onClick={() => {
              PageAction.removeNode(data);
            }}
          >
            {" "}
            删除{" "}
          </arco.Menu.Item>
        ) : null}
        {data.dataStatus === "0" ? (
          <arco.Menu.Item
            key="enable"
            onClick={() => {
              PageAction.enableNode(data);
            }}
          >
            启用
          </arco.Menu.Item>
        ) : null}
        {data.dataStatus === "1" ? (
          <arco.Menu.Item
            key="disable"
            onClick={() => {
              PageAction.disableNode(data);
            }}
          >
            停用
          </arco.Menu.Item>
        ) : null}
        <arco.Menu.Item
          key="changeParent"
          onClick={() => {
            PageAction.changeParentNode(data);
          }}
        >
          变更父级
        </arco.Menu.Item>
      </arco.Menu>
    );
  };
  const renderTreeNodExtra = (node) => {
    return (
      <arco.Dropdown
        droplist={<TreeNodExtraAction dataRef={node.dataRef} />}
        position="bl"
      >
        <arcoicon.IconMore
          style={{
            position: "absolute",
            right: 8,
            fontSize: 12,
            top: 10,
            color: "#3370ff",
          }}
        />
      </arco.Dropdown>
    );
  };
  return (
    <div>
      <div style={{ display: "block" }}>
        <arco.Input.Search
          searchButton
          placeholder="请输入"
          onSearch={(v) => {
            PageAction.findTreeData(v);
          }}
        />
      </div>
      <arco.Tree
        treeData={PageState.treeData}
        blockNode={true}
        renderExtra={renderTreeNodExtra}
        onSelect={(keys, extra) => {
          PageState.setSelectTreeNodeKeys(keys);
          PageState.setSelectTreeNodeData(extra.node.props.dataRef);
        }}
        showLine={true}
        selectedKeys={PageState.selectTreeNodeKeys}
        fieldNames={{
          key: "roleCode",
          title: "roleName",
          children: "roleChildren",
        }}
      ></arco.Tree>
    </div>
  );
};

const PageGridView = () => {
  return (
    <div className="bmbp-grid-container">
      <PageGridSearchForm />
      <PageGridToolBar />
      <PageGridTable />
      <PageGridPage />
    </div>
  );
};
const PageGridSearchForm = () => {
  return (
    <div className="bmbp-grid-search">
      <arco.Form colon ref={PageState.searchFormRef}>
        <arco.Grid.Row guides={[1, 1]}>
          <arco.Grid.Col span={7}>
            <arco.Form.Item label="编码" field="roleCode">
              <arco.Input placeholder="请输入编码" />
            </arco.Form.Item>
          </arco.Grid.Col>
          <arco.Grid.Col span={7}>
            <arco.Form.Item label="名称" field="roleName">
              <arco.Input placeholder="请输入名称" />
            </arco.Form.Item>
          </arco.Grid.Col>
          <arco.Grid.Col span={7}>
            <arco.Form.Item label="状态" field="dataStatus">
              <arco.Select placeholder="请选择状态">
                <arco.Select.Option key={"1"} value={"1"}>
                  已启用
                </arco.Select.Option>
                <arco.Select.Option key={"0"} value={"0"}>
                  已停用
                </arco.Select.Option>
              </arco.Select>
            </arco.Form.Item>
          </arco.Grid.Col>
          <arco.Grid.Col span={3}>
            <arco.Form.Item>
              <arco.Space>
                <arco.Button
                  type="primary"
                  style={{ marginLeft: "8px" }}
                  onClick={() => {
                    let formData =
                      PageState.searchFormRef.current.getFieldsValue();
                    PageState.setSearchFormData(formData);
                  }}
                >
                  查询
                </arco.Button>
                <arco.Button
                  onClick={() => {
                    PageState.searchFormRef.current.resetFields();
                  }}
                >
                  清空
                </arco.Button>
              </arco.Space>
            </arco.Form.Item>
          </arco.Grid.Col>
        </arco.Grid.Row>
      </arco.Form>
      <arco.Divider style={{ margin: "0px 0 4px 0 " }} />
    </div>
  );
};
const PageGridToolBar = () => {
  return (
    <div className="bmbp-grid-toolbar">
      <div className="bmbp-grid-toolbar major">
        <arco.Button
          type="primary"
          onClick={() => {
            PageAction.addChildNode(PageState.selectTreeNodeData?.dataRef);
          }}
        >
          新增
        </arco.Button>
        {PageState.selectedRowKeys && PageState.selectedRowKeys.length > 0 ? (
          <arco.Button
            type="primary"
            status="danger"
            onClick={() => {
              PageAction.batchRemoveNode(PageState.selectedRowKeys);
            }}
          >
            删除
          </arco.Button>
        ) : null}
      </div>
      <div className="bmbp-grid-toolbar extra">
        <arco.Button>导入</arco.Button>
        <arco.Button>导出</arco.Button>
      </div>
    </div>
  );
};
const PageGridTable = () => {
  const enableAction = (record: any) => {
    return [
      <arco.Tooltip content="新增子级">
        <arco.Button
          type="primary"
          icon={<arcoicon.IconPlus />}
          size="mini"
          onClick={() => {
            PageAction.addChildNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="查看">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconEye />}
          size="mini"
          onClick={() => {
            PageAction.viewInfo(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="停用">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconStop />}
          size="mini"
          onClick={() => {
            PageAction.disableNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="互斥角色">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconStrikethrough />}
          size="mini"
          onClick={() => {}}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="变更上级">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconStrikethrough />}
          size="mini"
          onClick={() => {
            PageAction.changeParentNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
    ];
  };
  const disableAction = (record: any) => {
    return [
      <arco.Tooltip content="新增子级">
        <arco.Button
          type="primary"
          icon={<arcoicon.IconPlus />}
          size="mini"
          onClick={() => {
            PageAction.addChildNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="编辑">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconEdit />}
          size="mini"
          onClick={() => {
            PageAction.editNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="启用">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconPlayArrow />}
          size="mini"
          onClick={() => {
            PageAction.enableNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="变更上级">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconStrikethrough />}
          size="mini"
          onClick={() => {
            PageAction.changeParentNode(record);
          }}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Tooltip content="互斥角色">
        <arco.Button
          type="secondary"
          icon={<arcoicon.IconStrikethrough />}
          size="mini"
          onClick={() => {}}
        ></arco.Button>
      </arco.Tooltip>,
      <arco.Popconfirm
        focusLock
        title="删除确认"
        content="数据删除后无法恢复，确定删除吗?"
        onOk={() => {
          PageAction.removeNode(record);
        }}
        onCancel={() => {}}
      >
        <arco.Tooltip content="删除">
          <arco.Button
            type="primary"
            status="danger"
            icon={<arcoicon.IconDelete />}
            size="mini"
          ></arco.Button>
        </arco.Tooltip>
      </arco.Popconfirm>,
    ];
  };
  const gridColumn = [
    {
      width: 300,
      ellipsis: true,
      title: "名称",
      dataIndex: "roleName",
    },
    {
      width: 120,
      ellipsis: true,
      title: "类型",
      dataIndex: "roleType",
    },
    {
      width: 120,
      ellipsis: true,
      title: "管理角色",
      dataIndex: "isAdmin",
    },
    {
      ellipsis: true,
      title: "描述",
      dataIndex: "roleDesc",
    },
    {
      title: "状态",
      dataIndex: "dataStatus",
      fixed: "right",
      width: 80,
      render: (value: any) => {
        if (value === "1") {
          return <arco.Tag color="green">已启用</arco.Tag>;
        } else {
          return <arco.Tag color="red">已停用</arco.Tag>;
        }
      },
    },
    {
      title: "操作",
      dataIndex: "action",
      width: 220,
      fixed: "right",
      align: "left",
      render: (value, record, index) => {
        return (
          <arco.Space>
            {record.dataStatus === "1"
              ? enableAction(record)
              : disableAction(record)}
          </arco.Space>
        );
      },
    },
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
    onSelect: (selected, record, selectedRows) => {},
  };
  return (
    <div className="bmbp-grid-table">
      <arco.Table
        rowKey="dataId"
        columns={gridColumn}
        data={PageState.gridData}
        rowSelection={gridRowSelection}
        pagination={false}
        stripe={true}
        border={{
          wrapper: true,
          cell: true,
        }}
      />
    </div>
  );
};
const PageGridPage = () => {
  return (
    <div className="bmbp-grid-page">
      <arco.Pagination
        size={PageState.showPageSize}
        total={PageState.pageData.total}
        showTotal
        showJumper
        sizeCanChange
        onChange={(pageNo, pageSize) => {
          PageState.setPageData({
            ...PageState.pageData,
            pageNo: pageNo,
            pageSize: pageSize,
          });
        }}
      />
    </div>
  );
};
