// web/tsx/app_group/action.tsx
var PageState = {};
var PageAction = {
  init: () => {
    const [treeData, setTreeData] = React.useState([]);
    PageState.treeData = treeData;
    PageState.setTreeData = setTreeData;
  }
};

// web/tsx/app_group/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(PageView, null));
};
var PageView = () => {
  PageAction.init();
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_container"
  }, /* @__PURE__ */ React.createElement(TreeGridView, null, /* @__PURE__ */ React.createElement(TreePanel, null, /* @__PURE__ */ React.createElement(TreePanelHeader, null), /* @__PURE__ */ React.createElement(TreeSearchBody, null), /* @__PURE__ */ React.createElement(TreePanelBody, null)), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical",
    style: { width: "1px", height: "100%", margin: "0 2px" }
  }), /* @__PURE__ */ React.createElement(GridView, null)));
};
var TreeGridView = (props) => {
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
  }, props.children);
};
var TreePanelHeader = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "tree_header"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "tree_title"
  }, "应用分组"), /* @__PURE__ */ React.createElement("div", {
    className: "tree_action"
  }, /* @__PURE__ */ React.createElement(BmbpIconFont, {
    type: "icon-zhongzhi"
  }), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical"
  }), /* @__PURE__ */ React.createElement(BmbpIconFont, {
    type: "icon-jia1"
  })));
};
var TreeSearchBody = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "tree_search"
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "请输入应用分组名称",
    style: { width: "100%" }
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
      className: "menu_item"
    }, "新增兄弟节点"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "addChildrenNode",
      className: "menu_item"
    }, "新增子节点"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "editNode",
      className: "menu_item"
    }, "编辑"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "changeParent",
      className: "menu_item"
    }, "变更上级"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "disableNode",
      className: "menu_item"
    }, "停用"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "enableNode",
      className: "menu_item"
    }, "启用"), !nodeData.isLeaf ? null : /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "deleteNode",
      className: "menu_item"
    }, "删除")));
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
  }, props.children);
};
var GridView = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_page_grid"
  }, "grid");
};
