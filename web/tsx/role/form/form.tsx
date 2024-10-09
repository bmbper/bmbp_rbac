import { PageAction, PageState } from "../action";

export const RoleForm = () => {
  React.useEffect(() => {
    if (PageState.currentRoleData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentRoleData);
    }
  }, [PageState.currentRoleData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="上级角色编码" field="roleParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="角色名称"
        field="roleName"
        rules={[{ required: true, message: "请输入角色名称" }]}
      >
        <arco.Input placeholder="请输入角色名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="角色编码"
        field="roleCode"
        rules={[{ required: true, message: "请输入角色编码" }]}
      >
        <arco.Input placeholder="请输入角色别名" />
      </arco.Form.Item>
      <arco.Form.Item label="角色类型" field="roleType">
        <arco.Input placeholder="请输入角色值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="roleOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};
export const RoleInfoForm = () => {
  React.useEffect(() => {
    if (PageState.currentRoleData) {
      PageState.addFormRef.current?.setFieldsValue(PageState.currentRoleData);
    }
  }, [PageState.currentRoleData]);
  return (
    <arco.Form ref={PageState.addFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="上级角色编码" field="roleParentCode" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item
        label="角色名称"
        field="roleName"
        rules={[{ required: true, message: "请输入角色名称" }]}
      >
        <arco.Input placeholder="请输入角色名称" />
      </arco.Form.Item>
      <arco.Form.Item
        label="角色编码"
        field="roleCode"
        rules={[{ required: true, message: "请输入角色编码" }]}
      >
        <arco.Input placeholder="请输入角色编码" />
      </arco.Form.Item>
      <arco.Form.Item label="角色类型" field="roleType">
        <arco.Input placeholder="请输入角色值" />
      </arco.Form.Item>
      <arco.Form.Item label="排序" field="roleOrder">
        <arco.InputNumber placeholder="请输入顺序" />
      </arco.Form.Item>
    </arco.Form>
  );
};

export const RoleParentForm = () => {
  React.useEffect(() => {
    if (
      PageState.waitChangeRoleDataId &&
      PageState.changeParentFormDialogVisible
    ) {
      PageAction.findTreeDataIgnoreDataId(PageState.waitChangeRoleDataId);
      PageState.changeParentFormRef?.current.setFieldsValue({
        dataId: PageState.waitChangeRoleDataId,
      });
    }
  }, [PageState.waitChangeRoleDataId]);
  return (
    <arco.Form ref={PageState.changeParentFormRef}>
      <arco.Form.Item label="主键" field="dataId" hidden={true}>
        <arco.Input placeholder="" />
      </arco.Form.Item>
      <arco.Form.Item label="目标角色" field="roleParentCode" hidden={false}>
        <arco.TreeSelect
          treeData={PageState.parentTreeData}
          fieldNames={{
            key: "roleCode",
            title: "roleName",
            children: "roleChildren",
          }}
          placeholder="请选择目标角色"
        />
      </arco.Form.Item>
    </arco.Form>
  );
};
