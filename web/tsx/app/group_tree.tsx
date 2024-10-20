import { PageAction, PageState } from './action';

export const PageTreeView = () => {
	return (
		<div>
			<div style={{ display: 'block' }}>
				<arco.Input.Search
					searchButton
					placeholder='请输入'
					onSearch={(v) => {
						PageAction.findTreeData(v);
					}}
				/>
			</div>
			<arco.Tree
				treeData={PageState.treeData}
				blockNode={true}
				onSelect={(keys, extra) => {
					PageState.setSelectTreeNodeKeys(keys);
					PageState.setSelectTreeNodeData(extra.node.props.dataRef);
				}}
				showLine={true}
				selectedKeys={PageState.selectTreeNodeKeys}
				fieldNames={{
					key: 'appGroupCode',
					title: 'appGroupName',
					children: 'appGroupChildren',
				}}></arco.Tree>
		</div>
	);
};
