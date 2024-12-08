import {PageAction, PageState} from "./action";
import {AddForm, EditForm, DetailForm, OwnerForm} from "./form";

export const AddFormDialog = () => {
    return (
        <>
            <arco.Modal
                title="新增分组"
                style={{width: "800px"}}
                visible={PageState.addFormVisible}
                onOk={() => {
                    PageState.addFormRef.current?.validate().then((data: any) => {
                        PageAction.saveData(data, () => {
                            PageAction.findTreeData();
                            PageState.setFormData(null);
                            PageState.addFormRef.current?.resetFields();
                            PageState.setAddFormVisible(false);
                        });
                    });
                }}
                onCancel={() => {
                    PageState.setFormData(null)
                    PageState.addFormRef.current?.resetFields();
                    PageState.setAddFormVisible(false)
                }}
            >
                <AddForm/>
            </arco.Modal>
        </>);
}
export const EditFormDialog = () => {
    return (
        <>
            <arco.Modal
                title="编辑分组"
                style={{width: "800px"}}
                visible={PageState.editFormVisible}
                onOk={() => {
                    PageState.editFormRef.current?.validate().then((data: any) => {
                        PageAction.saveData(data, () => {
                            PageAction.findTreeData();
                            PageState.setFormData(null);
                            PageState.editFormRef.current?.resetFields();
                            PageState.setEditFormVisible(false);
                        });
                    });
                }}
                onCancel={() => {
                    PageState.setFormData(null)
                    PageState.editFormRef.current?.resetFields();
                    PageState.setEditFormVisible(false)
                }}
            >
                <EditForm/>
            </arco.Modal>
        </>);
}
export const DetailFormDialog = () => {
    return (
        <>
            <arco.Modal
                title="查看分组"
                style={{width: "800px"}}
                visible={PageState.detailFormVisible}
                footer={null}
                onCancel={() => {
                    PageState.setFormData(null)
                    PageState.detailFormRef.current?.resetFields();
                    PageState.setDetailFormVisible(false)
                }}
            >
                <DetailForm/>
            </arco.Modal>
        </>);
}

export const OwnerFormDialog = () => {
    return (
        <>
            <arco.Modal
                title="变更所属分组"
                style={{width: "800px"}}
                visible={PageState.changeOwnerFormVisible}
                onOk={() => {
                    PageState.changeOwnerFormRef.current?.validate().then((data: any) => {
                        PageAction.saveData(data, () => {
                            PageAction.findTreeData();
                            PageState.setFormData(null);
                            PageState.changeOwnerFormRef.current?.resetFields();
                            PageState.setChangeOwnerFormVisible(false);
                        });
                    });
                }}
                onCancel={() => {
                    PageState.setFormData(null)
                    PageState.changeOwnerFormRef.current?.resetFields();
                    PageState.setChangeOwnerFormVisible(false)
                }}
            >
                <OwnerForm/>
            </arco.Modal>
        </>);
}
