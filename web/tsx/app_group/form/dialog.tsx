import { PageAction, PageState } from '../action';
import { AppGroupForm, AppGroupInfoForm, AppGroupParentForm } from './form';

export const AddAppGroupFormDialog = () => {
	return (
		<arco.Modal
			title='新增组织'
			visible={PageState.addFormDialogVisible}
			onOk={() => {
				PageState.addFormRef.current?.validate().then((data: any) => {
					PageAction.save(data, () => {
						PageState.setCurrentAppGroupData(null);
						PageState.setAddFormDialogVisible(false);
						PageState.addFormRef.current?.resetFields();
						PageAction.findTreeData(null);
					});
				});
			}}
			onCancel={() => {
				PageState.addFormRef.current?.resetFields();
				PageState.setAddFormDialogVisible(false);
			}}>
			<AppGroupForm />
		</arco.Modal>
	);
};
export const EditAppGroupFormDialog = () => {
	return (
		<>
			<arco.Modal
				title='编辑组织'
				visible={PageState.editFormDialogVisible}
				onOk={() => {
					PageState.addFormRef.current?.validate().then((data: any) => {
						PageAction.save(data, () => {
							PageState.setCurrentAppGroupData(null);
							PageState.setEditFormDialogVisible(false);
							PageState.addFormRef.current?.resetFields();
							PageAction.findTreeData(null);
						});
					});
				}}
				onCancel={() => {
					PageState.editFormRef.current?.resetFields();
					PageState.setEditFormDialogVisible(false);
				}}>
				<AppGroupForm />
			</arco.Modal>
		</>
	);
};
export const InfoAppGroupFormDialog = () => {
	return (
		<arco.Modal title='查看组织' visible={PageState.infoFormDialogVisible} onCancel={() => PageState.setInfoFormDialogVisible(false)} footer={null}>
			<AppGroupInfoForm />
		</arco.Modal>
	);
};
export const ChangeParentAppGroupFormDialog = () => {
	return (
		<arco.Modal
			title='变更上级'
			visible={PageState.changeParentFormDialogVisible}
			onOk={() => {
				PageState.changeParentFormRef.current?.validate().then((data: any) => {
					debugger;
					PageAction.updateParent(data, () => {
						PageState.setChangeParentFormDialogVisible(false);
						PageState.changeParentFormRef.current?.resetFields();
						PageAction.findTreeData('');
					});
				});
			}}
			onCancel={() => {
				PageState.changeParentFormRef.current?.resetFields();
				PageState.setChangeParentFormDialogVisible(false);
			}}>
			<AppGroupParentForm />
		</arco.Modal>
	);
};

export const ImportAppGroupFormDialog = () => {
	return (
		<>
			<arco.Modal
				title='导入组织'
				visible={PageState.importFormDialogVisible}
				onOk={() => PageState.setImportFormDialogVisible(false)}
				onCancel={() => PageState.setImportFormDialogVisible(false)}></arco.Modal>
		</>
	);
};
export const ExportAppGroupFormDialog = () => {
	return (
		<>
			<arco.Modal
				title='导出组织'
				visible={PageState.exportFormDialogVisible}
				onOk={() => PageState.setExportFormDialogVisible(false)}
				onCancel={() => PageState.setExportFormDialogVisible(false)}></arco.Modal>
		</>
	);
};
