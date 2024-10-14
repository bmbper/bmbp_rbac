import { PageAction, PageState } from "../action";

export const AppGroupForm = () => {
  React.useEffect(() => {
    if (PageState.currentAppGroupData) {
      PageState.addFormRef.current?.setFieldsValue(
        PageState.currentAppGroupData
      );
    }
  }, [PageState.currentAppGroupData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="上级组织编码"
        field="app_groupParentCode"
        hidden={true}
      >
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="app_groupName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="app_groupCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织别名" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="app_groupType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="app_groupOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};
export const AppGroupInfoForm = () => {
  React.useEffect(() => {
    if (PageState.currentAppGroupData) {
      PageState.addFormRef.current?.setFieldsValue(
        PageState.currentAppGroupData
      );
    }
  }, [PageState.currentAppGroupData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="上级组织编码"
        field="app_groupParentCode"
        hidden={true}
      >
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="app_groupName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="app_groupCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织编码" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="app_groupType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="app_groupOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};

export const AppGroupParentForm = () => {
  React.useEffect(() => {
    if (
      PageState.waitChangeAppGroupDataId &&
      PageState.changeParentFormDialogVisible
    ) {
      PageAction.findTreeDataIgnoreDataId(PageState.waitChangeAppGroupDataId);
      PageState.changeParentFormRef?.current.setFieldsValue({
        dataId: PageState.waitChangeAppGroupDataId,
      });
    }
  }, [PageState.waitChangeAppGroupDataId]);
  return (
    <arco.Form ref={PageState.changeParentFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="目标组织"
        field="app_groupParentCode"
        hidden={false}
      >
        <arco.TreeSelect
          treeData={PageState.parentTreeData}
          fieldNames={{
            key: "app_groupCode",
            title: "app_groupName",
            children: "app_groupChildren",
          }}
          placeholder="请选择目标组织"
        />
      </arco.Form.Item>
    </arco.Form>
  );
};
