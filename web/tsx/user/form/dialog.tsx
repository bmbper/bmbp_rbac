import { PageAction, PageState } from "../action";
import { UserForm, UserInfoForm, UserParentForm } from "./form";

export const AddUserFormDialog = () => {
  return (
    <arco.Modal
      title="新增组织"
      visible={PageState.addFormDialogVisible}
      onOk={() => {
        PageState.addFormRef.current?.validate().then((data: any) => {
          PageAction.save(data, () => {
            PageState.setCurrentUserData(null);
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
      <UserForm />
    </arco.Modal>
  );
};
export const EditUserFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="编辑组织"
        visible={PageState.editFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentUserData(null);
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
        <UserForm />
      </arco.Modal>
    </>
  );
};
export const InfoUserFormDialog = () => {
  return (
    <arco.Modal
      title="查看组织"
      visible={PageState.infoFormDialogVisible}
      onOk={() => PageState.setInfoFormDialogVisible(false)}
      onCancel={() => PageState.setInfoFormDialogVisible(false)}
    >
      <UserInfoForm />
    </arco.Modal>
  );
};
export const ChangeParentUserFormDialog = () => {
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
      <UserParentForm />
    </arco.Modal>
  );
};

export const ImportUserFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="导入组织"
        visible={PageState.importFormDialogVisible}
        onOk={() => PageState.setImportFormDialogVisible(false)}
        onCancel={() => PageState.setImportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
export const ExportUserFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="导出组织"
        visible={PageState.exportFormDialogVisible}
        onOk={() => PageState.setExportFormDialogVisible(false)}
        onCancel={() => PageState.setExportFormDialogVisible(false)}
      ></arco.Modal>
    </>
  );
};
