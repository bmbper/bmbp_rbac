import { PageAction, PageState } from "../action";
import { RoleForm, RoleInfoForm, RoleParentForm } from "./form";

export const AddRoleFormDialog = () => {
  return (
    <arco.Modal
      title="新增角色"
      visible={PageState.addFormDialogVisible}
      onOk={() => {
        PageState.addFormRef.current?.validate().then((data: any) => {
          PageAction.save(data, () => {
            debugger;
            PageState.setCurrentRoleData(null);
            PageState.setAddFormDialogVisible(false);
            PageState.addFormRef.current?.resetFields();
            PageAction.findTreeData(null);
          });
        });
      }}
      onCancel={() => {
        PageState.addFormRef.current?.resetFields();
        PageState.setAddFormDialogVisible(false);
      }}
    >
      <RoleForm />
    </arco.Modal>
  );
};
export const EditRoleFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="编辑角色"
        visible={PageState.editFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentRoleData(null);
              PageState.setEditFormDialogVisible(false);
              PageState.addFormRef.current?.resetFields();
              PageAction.findTreeData(null);
            });
          });
        }}
        onCancel={() => {
          PageState.editFormRef.current?.resetFields();
          PageState.setEditFormDialogVisible(false);
        }}
      >
        <RoleForm />
      </arco.Modal>
    </>
  );
};
export const InfoRoleFormDialog = () => {
  return (
    <arco.Modal
      title="查看角色"
      visible={PageState.infoFormDialogVisible}
      onOk={() => PageState.setInfoFormDialogVisible(false)}
      onCancel={() => PageState.setInfoFormDialogVisible(false)}
    >
      <RoleInfoForm />
    </arco.Modal>
  );
};
export const ChangeParentRoleFormDialog = () => {
  return (
    <arco.Modal
      title="变更上级"
      visible={PageState.changeParentFormDialogVisible}
      onOk={() => {
        PageState.changeParentFormRef.current?.validate().then((data: any) => {
          debugger;
          PageAction.updateParent(data, () => {
            PageState.setChangeParentFormDialogVisible(false);
            PageState.changeParentFormRef.current?.resetFields();
            PageAction.findTreeData("");
          });
        });
      }}
      onCancel={() => {
        PageState.changeParentFormRef.current?.resetFields();
        PageState.setChangeParentFormDialogVisible(false);
      }}
    >
      <RoleParentForm />
    </arco.Modal>
  );
};

export const ImportRoleFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="导入角色"
        visible={PageState.importFormDialogVisible}
        onOk={() => PageState.setImportFormDialogVisible(false)}
        onCancel={() => PageState.setImportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
export const ExportRoleFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="导出角色"
        visible={PageState.exportFormDialogVisible}
        onOk={() => PageState.setExportFormDialogVisible(false)}
        onCancel={() => PageState.setExportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
