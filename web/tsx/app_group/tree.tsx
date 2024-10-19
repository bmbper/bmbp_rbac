import { PageAction, PageState } from "./action";

export const PageTreeView = () => {
  const TreeNodExtraAction = (props) => {
    const data = props.dataRef;
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
        <arco.Popconfirm
				focusLock
				title='删除确认'
				content='该数据及子数据删除后无法恢复，确定删除吗?'
				onOk={() => {
					PageAction.removeNode(data);
				}}
				onCancel={() => {}}>
					<arco.Button type='primary'>
								删除
					</arco.Button>
			</arco.Popconfirm>,
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
          key: "appGroupCode",
          title: "appGroupName",
          children: "appGroupChildren",
        }}
      ></arco.Tree>
    </div>
  );
};
