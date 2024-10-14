import { PageAction, PageState } from "../action";

export const MenuForm = () => {
  React.useEffect(() => {
    if (PageState.currentMenuData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentMenuData);
    }
  }, [PageState.currentMenuData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="上级组织编码" field="menuParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="menuName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="menuCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织别名" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="menuType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="menuOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};
export const MenuInfoForm = () => {
  React.useEffect(() => {
    if (PageState.currentMenuData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentMenuData);
    }
  }, [PageState.currentMenuData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="上级组织编码" field="menuParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="menuName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="menuCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织编码" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="menuType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="menuOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};

export const MenuParentForm = () => {
  React.useEffect(() => {
    if (
      PageState.waitChangeMenuDataId &&
      PageState.changeParentFormDialogVisible
    ) {
      PageAction.findTreeDataIgnoreDataId(PageState.waitChangeMenuDataId);
      PageState.changeParentFormRef?.current.setFieldsValue({
        dataId: PageState.waitChangeMenuDataId,
      });
    }
  }, [PageState.waitChangeMenuDataId]);
  return (
    <arco.Form ref={PageState.changeParentFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="目标组织" field="menuParentCode" hidden={false}>
        <arco.TreeSelect
          treeData={PageState.parentTreeData}
          fieldNames={{
            key: "menuCode",
            title: "menuName",
            children: "menuChildren",
          }}
          placeholder="请选择目标组织"
        />
      </arco.Form.Item>
    </arco.Form>
  );
};
