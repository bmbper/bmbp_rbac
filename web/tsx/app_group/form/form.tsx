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
        label="上级分组编码"
        field="appGroupParentCode"
        hidden={true}
      >
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="分组名称"
        field="appGroupName"
        rules={[{ required: true, message: "请输入分组名称" }]}
      >
        <arco.Input placeholder="请输入分组名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="分组编码"
        field="appGroupCode"
        rules={[{ required: true, message: "请输入分组编码" }]}
      >
        <arco.Input placeholder="请输入分组别名" />
      </arco.Form.Item>
      <arco.Form.Item label="分组类型" field="appGroupType">
        <arco.Input placeholder="请输入分组值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="appGroupOrder">
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
        label="上级分组编码"
        field="appGroupParentCode"
        hidden={true}
      >
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="分组名称"
        field="appGroupName"
        rules={[{ required: true, message: "请输入分组名称" }]}
      >
        <arco.Input placeholder="请输入分组名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="分组编码"
        field="appGroupCode"
        rules={[{ required: true, message: "请输入分组编码" }]}
      >
        <arco.Input placeholder="请输入分组编码" />
      </arco.Form.Item>
      <arco.Form.Item label="分组类型" field="appGroupType">
        <arco.Input placeholder="请输入分组值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="appGroupOrder">
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
        label="目标分组"
        field="appGroupParentCode"
        hidden={false}
      >
        <arco.TreeSelect
          treeData={PageState.parentTreeData}
          fieldNames={{
            key: "appGroupCode",
            title: "appGroupName",
            children: "appGroupChildren",
          }}
          placeholder="请选择目标分组"
        />
      </arco.Form.Item>
    </arco.Form>
  );
};
