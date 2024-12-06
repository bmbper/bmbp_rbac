import {PageAction, PageState} from "./action";

window.onload = () => {
    const root = ReactDOM.createRoot(document.getElementById("app"));
    root.render(<PageView/>);
}

const PageView = () => {
    PageAction.init();
    return <div className={"bmbp_container"}>
        <TreeGridView>
            <TreePanel>
                <TreePanelHeader/>
                <TreeSearchBody/>
                <TreePanelBody/>
            </TreePanel>
            <arco.Divider type={'vertical'} style={{width: "1px", height: "100%", margin: "0 2px"}}/>
            <GridView/>
        </TreeGridView>
    </div>
}

const TreeGridView = (props: any) => {
    React.useEffect(() => {
        const treeData = [
            {
                title: 'Trunk 0-0',
                key: '0-0',
                children: [
                    {
                        title: 'Branch 0-0-2',
                        key: '0-0-2',
                        selectable: false,
                        children: [
                            {
                                title: 'Leaf',
                                key: '0-0-2-1',
                                children: [
                                    {
                                        title: 'Leafsss 0-0-2',
                                        key: '0-0-2-1-0',
                                        children: [
                                            {
                                                title: 'Leaf',
                                                key: '0-0-2-1-0-0',
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            {
                title: 'Trunk 0-1',
                key: '0-1',
                children: [
                    {
                        title: 'Branch 0-1-1',
                        key: '0-1-1',
                        children: [
                            {
                                title: 'Leaf',
                                key: '0-1-1-0',
                            },
                        ],
                    },
                ],
            },
        ];
        PageState.setTreeData(treeData);
    }, [])
    return <div className={"bmbp_page_tree_grid"}>
        {props.children}
    </div>
}
const TreePanelHeader = () => {
    return (
        <div className={"tree_header"}>
            <div className={"tree_title"}>
                应用分组
            </div>
            <div className={"tree_action"}>
                <BmbpIconFont type={"icon-zhongzhi"}/>
                <arco.Divider type={'vertical'}/>
                <BmbpIconFont type={"icon-jia1"}/>
            </div>
        </div>
    )
}

const TreeSearchBody = () => {
    return (
        <div className={"tree_search"}>
            <arco.Input placeholder={"请输入应用分组名称"} style={{width: "100%"}}/>
        </div>
    )
}
const TreePanelBody = () => {
    const generatorTreeNodes = (treeData) => {
        return treeData.map((item) => {
            const {children, key, ...rest} = item;
            return (
                <arco.Tree.Node key={key} {...rest} dataRef={item}>
                    {children ? generatorTreeNodes(item.children) : null}
                </arco.Tree.Node>
            );
        });
    };
    const TreeNodeAction = (props: any) => {
        const nodeData = props.node.dataRef;
        const dropList = (
            <>
                <arco.Menu className={"tree_node_action_menu"}>
                    <arco.Menu.Item key="addBrotherNode" className={"menu_item"}>
                        新增兄弟节点
                    </arco.Menu.Item>
                    <arco.Menu.Item key="addChildrenNode" className={"menu_item"}>
                        新增子节点
                    </arco.Menu.Item>
                    <arco.Menu.Item key="editNode" className={"menu_item"}>
                        编辑
                    </arco.Menu.Item>
                    <arco.Menu.Item key="changeParent" className={"menu_item"}>
                        变更上级
                    </arco.Menu.Item>
                    <arco.Menu.Item key="disableNode" className={"menu_item"}>
                        停用
                    </arco.Menu.Item>
                    <arco.Menu.Item key="enableNode" className={"menu_item"}>
                        启用
                    </arco.Menu.Item>
                    {
                        !nodeData.isLeaf ? null : (
                            <arco.Menu.Item key="deleteNode" className={"menu_item"}>
                                删除
                            </arco.Menu.Item>
                        )
                    }
                </arco.Menu>
            </>
        );
        return (<div className={"tree_node_action"}>
            <arco.Dropdown droplist={dropList} position='br'>
                <BmbpIconFont type={"icon-zuoyouduiqi"}/>
            </arco.Dropdown>
        </div>)
    }
    return (
        <div className={"tree_body"}>
            <arco.Tree blockNode renderExtra={(node) => {
                return (<TreeNodeAction node={node}/>);
            }}>
                {generatorTreeNodes(PageState.treeData)}
            </arco.Tree>
        </div>
    )
}

const TreePanel = (props: any) => {
    return (
        <div className={"bmbp_page_tree"} style={{width: "300px"}}>
            {props.children}
        </div>
    )
}
const GridView = () => {
    return (<div className={"bmbp_page_grid"}>
        grid
    </div>)
}
