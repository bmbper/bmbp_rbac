import { PageAction, PageState } from './action';
import { AddAppGroupFormDialog, ChangeParentAppGroupFormDialog, EditAppGroupFormDialog, ExportAppGroupFormDialog, ImportAppGroupFormDialog, InfoAppGroupFormDialog } from './form/dialog';
import { PageGridView } from './grid';
import { PageTreeView } from './group_tree';

window.onload = () => {
	const root = ReactDOM.createRoot(document.getElementById('app'));
	root.render(<PageView />);
};
const PageView = (props) => {
	PageAction.init(props);
	React.useEffect(() => {
		PageAction.findTreeData(null);
	}, []);
	React.useEffect(() => {
		PageAction.findGridData();
	}, [PageState.pageData.pageNo, PageState.pageData.pageSize, PageState.selectTreeNodeData, PageState.searchFormData]);
	return (
		<div className='bmbp-app-fluid'>
			<arco.Grid.Row guides={[1, 1]} style={{ height: '100vh' }}>
				<arco.Grid.Col flex={'260px'}>
					<PageTreeView />
				</arco.Grid.Col>
				<arco.Divider type='vertical' style={{ height: '100%' }}></arco.Divider>
				<arco.Grid.Col flex={'auto'} style={{ height: '100%', width: '600px' }}>
					<PageGridView />
				</arco.Grid.Col>
			</arco.Grid.Row>
			<AddAppGroupFormDialog />
			<EditAppGroupFormDialog />
			<InfoAppGroupFormDialog />
			<ChangeParentAppGroupFormDialog />
			<ImportAppGroupFormDialog />
			<ExportAppGroupFormDialog />
		</div>
	);
};
