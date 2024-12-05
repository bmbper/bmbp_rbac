// web/tsx/app_group/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(PageView, null));
};
var PageView = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_container"
  }, /* @__PURE__ */ React.createElement(TreeGridView, null, /* @__PURE__ */ React.createElement(TreeView, null, /* @__PURE__ */ React.createElement(TreeViewHeader, null)), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical",
    style: { width: "1px", height: "100%", margin: "0 2px" }
  }), /* @__PURE__ */ React.createElement(GridView, null)));
};
var TreeGridView = (props) => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp_page_tree_grid"
  }, props.children);
};
var TreeViewHeader = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "tree_header"
  }, /* @__PURE__ */ React.createElement("div", {
    className: "tree_title"
  }, "应用分组"), /* @__PURE__ */ React.createElement("div", {
    className: "tree_action"
  }, "ddd"));
};
var TreeView = (props) => {
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
