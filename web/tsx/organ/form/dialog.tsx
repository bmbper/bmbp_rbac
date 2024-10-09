import { PageAction, PageState } from "../action";
import { OrganForm, OrganInfoForm, OrganParentForm } from "./form";

export const AddOrganFormDialog = () => {
  return (
    <arco.Modal
      title="新增组织"
      visible={PageState.addFormDialogVisible}
      onOk={() => {
        PageState.addFormRef.current?.validate().then((data: any) => {
          PageAction.save(data, () => {
            PageState.setCurrentOrganData(null);
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
      <OrganForm />
    </arco.Modal>
  );
};
export const EditOrganFormDialog = () => {
  return (
    <>
      <arco.Modal
        title="编辑组织"
        visible={PageState.editFormDialogVisible}
        onOk={() => {
          PageState.addFormRef.current?.validate().then((data: any) => {
            PageAction.save(data, () => {
              PageState.setCurrentOrganData(null);
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
        <OrganForm />
      </arco.Modal>
    </>
  );
};
export const InfoOrganFormDialog = () => {
  return (
    <arco.Modal
      title="查看组织"
      visible={PageState.infoFormDialogVisible}
      onOk={() => PageState.setInfoFormDialogVisible(false)}
      onCancel={() => PageState.setInfoFormDialogVisible(false)}
    >
      <OrganInfoForm />
    </arco.Modal>
  );
};
export const ChangeParentOrganFormDialog = () => {
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
      <OrganParentForm />
    </arco.Modal>
  );
};

export const ImportOrganFormDialog = () => {
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
export const ExportOrganFormDialog = () => {
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
