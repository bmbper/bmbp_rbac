import { PageAction, PageState } from "../action";
import { MenuForm, MenuInfoForm, MenuParentForm } from "./form";

export const AddMenuFormDialog = () => {
  return (
    <arco.Modal
      title="新增组织"
      visible={PageState.addFormDialogVisible}
      onOk={() => {
        PageState.addFormRef.current?.validate().then((data: any) => {
          PageAction.save(data, () => {
            PageState.setCurrentMenuData(null);
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
      <MenuForm />
    </arco.Modal>
  );
};
export const EditMenuFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="编辑组织"
        visible={PageState.editFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentMenuData(null);
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
        <MenuForm />
      </arco.Modal>
    </>
  );
};
export const InfoMenuFormDialog = () => {
  return (
    <arco.Modal
      title="查看组织"
      visible={PageState.infoFormDialogVisible}
      onOk={() => PageState.setInfoFormDialogVisible(false)}
      onCancel={() => PageState.setInfoFormDialogVisible(false)}
    >
      <MenuInfoForm />
    </arco.Modal>
  );
};
export const ChangeParentMenuFormDialog = () => {
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
      <MenuParentForm />
    </arco.Modal>
  );
};

export const ImportMenuFormDialog = () => {
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
export const ExportMenuFormDialog = () => {
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
