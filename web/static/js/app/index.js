// web/tsx/app/index.tsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement("div", null, "App"));
};