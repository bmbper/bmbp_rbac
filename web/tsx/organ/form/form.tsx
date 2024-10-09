import { PageAction, PageState } from "../action";

export const OrganForm = () => {
  React.useEffect(() => {
    if (PageState.currentOrganData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentOrganData);
    }
  }, [PageState.currentOrganData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="上级组织编码"
        field="organParentCode"
        hidden={true}
      >
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="organName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="organCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织别名" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="organType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="organOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};
export const OrganInfoForm = () => {
  React.useEffect(() => {
    if (PageState.currentOrganData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentOrganData);
    }
  }, [PageState.currentOrganData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="上级组织编码"
        field="organParentCode"
        hidden={true}
      >
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="organName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="organCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织编码" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="organType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="organOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};

export const OrganParentForm = () => {
  React.useEffect(() => {
    if (
      PageState.waitChangeOrganDataId &&
      PageState.changeParentFormDialogVisible
    ) {
      PageAction.findTreeDataIgnoreDataId(PageState.waitChangeOrganDataId);
      PageState.changeParentFormRef?.current.setFieldsValue({
        dataId: PageState.waitChangeOrganDataId,
      });
    }
  }, [PageState.waitChangeOrganDataId]);
  return (
    <arco.Form ref={PageState.changeParentFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="目标组织" field="organParentCode" hidden={false}>
        <arco.TreeSelect
          treeData={PageState.parentTreeData}
          fieldNames={{
            key: "organCode",
            title: "organName",
            children: "organChildren",
          }}
          placeholder="请选择目标组织"
        />
      </arco.Form.Item>
    </arco.Form>
  );
};
