import { PageAction, PageState } from "../action";

export const UserForm = () => {
  React.useEffect(() => {
    if (PageState.currentUserData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentUserData);
    }
  }, [PageState.currentUserData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="上级组织编码" field="userParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="userName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="userCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织别名" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="userType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="userOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};
export const UserInfoForm = () => {
  React.useEffect(() => {
    if (PageState.currentUserData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentUserData);
    }
  }, [PageState.currentUserData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="上级组织编码" field="userParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织名称"
        field="userName"
        rules={[{ required: true, message: "请输入组织名称" }]}
      >
        <arco.Input placeholder="请输入组织名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="组织编码"
        field="userCode"
        rules={[{ required: true, message: "请输入组织编码" }]}
      >
        <arco.Input placeholder="请输入组织编码" />
      </arco.Form.Item>
      <arco.Form.Item label="组织类型" field="userType">
        <arco.Input placeholder="请输入组织值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="userOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};

export const UserParentForm = () => {
  React.useEffect(() => {
    if (
      PageState.waitChangeUserDataId &&
      PageState.changeParentFormDialogVisible
    ) {
      PageAction.findTreeDataIgnoreDataId(PageState.waitChangeUserDataId);
      PageState.changeParentFormRef?.current.setFieldsValue({
        dataId: PageState.waitChangeUserDataId,
      });
    }
  }, [PageState.waitChangeUserDataId]);
  return (
    <arco.Form ref={PageState.changeParentFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="目标组织" field="userParentCode" hidden={false}>
        <arco.TreeSelect
          treeData={PageState.parentTreeData}
          fieldNames={{
            key: "userCode",
            title: "userName",
            children: "userChildren",
          }}
          placeholder="请选择目标组织"
        />
      </arco.Form.Item>
    </arco.Form>
  );
};
