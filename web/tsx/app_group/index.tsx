import {PageAction, PageState} from "./action";
import {AddFormDialog, EditFormDialog, DetailFormDialog, OwnerFormDialog} from "./dialog";

window.onload = () => {
    const root = ReactDOM.createRoot(document.getElementById("app"));
    root.render(<PageView/>);
}
const PageView = () => {
    PageAction.init();
    React.useEffect(() => {
        PageAction.findTreeData(null);
    }, []);
    React.useEffect(() => {
        PageAction.findPageData();
    }, [
        PageState.pageData.pageNum, PageState.pageData.pageSize, PageState.treeSelectedNodeData, PageState.searchFormData
    ]);
    return <div className={"bmbp_container"}>
        <TreeGridPageView/>
    </div>
}

const TreeGridPageView = (props: any) => {
    return <div className={"bmbp_page_tree_grid"}>
        <TreePanel/>
        <arco.Divider type={'vertical'} style={{width: "1px", height: "100%", margin: "0 2px"}}/>
        <GridPanel/>
        <AddFormDialog/>
        <EditFormDialog/>
        <DetailFormDialog/>
        <OwnerFormDialog/>
    </div>
}
const TreeHeader = () => {
    return (
        <div className={"tree_header"}>
            <div className={"tree_title"}>
                应用分组
            </div>
            <div className={"tree_action"}>
                <BmbpIconFont type={"icon-zhongzhi"} onClick={() => {
                    PageState.setTreeSelectedNodeData(null);
                    PageState.setTreeSearchValue("");
                    PageAction.findTreeData();
                }}/>
                <arco.Divider type={'vertical'}/>
                <BmbpIconFont type={"icon-jia1"} onClick={() => {
                    PageAction.openAddBrotherForm(null);
                }}/>
            </div>
        </div>
    )
}
const TreeSearchBody = () => {
    return (
        <div className={"tree_search"}>
            <arco.Input placeholder={"请输入应用分组名称"} style={{width: "100%"}} onChange={(v) => {
                PageState.setTreeSearchValue(v);
            }}/>
        </div>
    )
}
const TreePanelBody = () => {
    const generatorTreeNodes = (treeData) => {
        return treeData.map((item) => {
            const {data_id, app_group_name, app_group_children, ...rest} = item;
            return (
                <arco.Tree.Node key={data_id} title={app_group_name} {...rest} dataRef={item}>
                    {app_group_children ? generatorTreeNodes(app_group_children) : null}
                </arco.Tree.Node>
            );
        });
    };
    const TreeNodeAction = (props: any) => {
        const nodeData = props.node.dataRef;
        const dropList = (
            <>
                <arco.Menu className={"tree_node_action_menu"}>
                    <arco.Menu.Item key="addBrotherNode" className={"menu_item"} onClick={() => {
                        PageAction.openAddBrotherForm(nodeData);
                    }}>
                        新增兄弟节点
                    </arco.Menu.Item>
                    <arco.Menu.Item key="addChildrenNode" className={"menu_item"} onClick={() => {
                        PageAction.openAddChildForm(nodeData);
                    }}>
                        新增子节点
                    </arco.Menu.Item>
                    {
                        nodeData.dataStatus === "0" ? (<>
                            <arco.Menu.Item key="editNode" className={"menu_item"} onClick={() => {
                                PageAction.openEditForm(nodeData);
                            }}>
                                编辑
                            </arco.Menu.Item>
                            <arco.Menu.Item key="changeParent" className={"menu_item"} onClick={() => {
                                PageAction.openChangeParentForm(nodeData);
                            }}>
                                变更上级
                            </arco.Menu.Item>
                            <arco.Menu.Item key="enableNode" className={"menu_item"} onClick={() => {
                                PageAction.enableData(nodeData);
                            }}>
                                启用
                            </arco.Menu.Item>
                            <arco.Menu.Item key="deleteNode" className={"menu_item"}>
                                <arco.Popconfirm
                                    focusLock
                                    title='删除确认'
                                    content='删除后数据将不再存在，是否删除?'
                                    onOk={() => {
                                        PageAction.removeData(record);
                                    }}
                                    onCancel={() => {
                                    }}>
                                    删除
                                </arco.Popconfirm>
                            </arco.Menu.Item>
                        </>) : (<>
                            <arco.Menu.Item key="disableNode" className={"menu_item"} onClick={() => {
                                PageAction.disableData(nodeData);
                            }}>
                                停用
                            </arco.Menu.Item>
                        </>)
                    }
                    <arco.Menu.Item key="infoNode" className={"menu_item"} onClick={() => {
                        PageAction.openInfoForm(nodeData);
                    }}>查看
                    </arco.Menu.Item>
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
            <arco.Tree blockNode
                       renderExtra={(node) => {
                           return (<TreeNodeAction node={node}/>);
                       }}
                       fieldNames={{
                           key: 'data_id',
                           title: 'app_group_name',
                           children: 'app_group_children',
                       }}
            >
                {generatorTreeNodes(PageState.treeData)}
            </arco.Tree>
        </div>
    )
}
const TreePanel = (props: any) => {
    return (
        <div className={"bmbp_page_tree"} style={{width: "300px"}}>
            <TreeHeader/>
            <TreeSearchBody/>
            <TreePanelBody/>
        </div>
    )
}
const GridPanel = () => {
    return (<div className={"bmbp_page_grid"}>
        <SearchForm/>
        <arco.Divider style={{margin: "4px 0 4px 0 "}}/>
        <GridToolBar/>
        <GridTable/>
        <GridPage/>
    </div>)
}
const SearchForm = (props: any) => {
    return (
        <div className={"search_form"}>
            <SearchFormBody/>
            <SearchFormAction/>
        </div>
    )
}
const SearchFormBody = (props: any) => {
    return (
        <div className={"search_body"}>
            <arco.Form ref={props.searchFormRef}>
                <arco.Grid.Row>
                    <arco.Grid.Col span={12}>
                        <arco.Form.Item label='分组名称' field='node_name'>
                            <arco.Input placeholder='请输入分组名称'/>
                        </arco.Form.Item>
                    </arco.Grid.Col>
                    <arco.Grid.Col span={12}>
                        <arco.Form.Item label='分组状态' field='data_status'>
                            <arco.Select placeholder='请选择分组状态'>
                                <arco.Option value="0" key="0">已停用</arco.Option>
                                <arco.Option value="1" key="1">已启用</arco.Option>
                            </arco.Select>
                        </arco.Form.Item>
                    </arco.Grid.Col>
                </arco.Grid.Row>
            </arco.Form>
        </div>
    )
}
const SearchFormAction = (props: any) => {
    return (
        <div className={"search_action"}>
            <arco.Button type={"primary"} onClick={() => {
                const searchFormData = PageState.searchFormRef.current?.getFieldsValue();
                PageState.setSearchFormData(searchFormData);
            }}>查询
            </arco.Button>
            <arco.Button onClick={() => {
                PageState.searchFormRef.current?.resetFields();
                PageState.setSearchFormData({});

            }}>重置
            </arco.Button>
        </div>
    )
}
const GridToolBar = () => {
    return (
        <div className={"grid_tool_bar"}>
            <arco.Button type={"primary"} onClick={() => {
                PageAction.openAddChildForm(PageState.treeSelectNodeData?.dataRef);
            }}>新增
            </arco.Button>
        </div>
    )
}
const GridTable = () => {
    const RowDisableAction = (record: any) => {
        return <>
            <arco.Button type={"text"} onClick={() => {
                PageAction.openEditForm(record);
            }}>
                编辑
            </arco.Button>
            <arco.Button type={"text"} onClick={() => {
                PageAction.enableData(record);
            }}>
                启用
            </arco.Button>
            <arco.Popconfirm
                focusLock
                title='删除确认'
                content='删除后数据将不再存在，是否删除?'
                onOk={() => {
                    PageAction.removeData(record);
                }}
                onCancel={() => {
                }}>
                <arco.Button type={"text"} status={'danger'}>
                    删除
                </arco.Button>
            </arco.Popconfirm>

        </>;
    }
    const RowEnableAction = (record: any) => {
        return <>
            <arco.Button type={"text"} onClick={() => {
                PageAction.disableData(record);
            }}>
                停用
            </arco.Button>
        </>;
    }
    const tableColumnConfig = [
        {
            title: '序号',
            dataIndex: '',
            render: (_, record, index) => {
                return PageState.pageData.pageSize * (PageState.pageData.pageNum - 1) + index + 1;
            },
            width: 80,
        },
        {
            title: '分组名称',
            dataIndex: 'node_name',
            width: 150,
        },
        {
            title: '分组标签',
            dataIndex: 'group_tag',
        },
        {
            title: '分组状态',
            dataIndex: 'data_status',
            width: 100,
            render: (value: any, record, index) => {
                return <arco.Tag
                    color={value === '1' ? 'green' : 'red'}>{value === '1' ? '已启用' : '已停用'}</arco.Tag>
            },
        },
        {
            title: '操作',
            width: 200,
            render: (value, record: any, index) => {
                return <div className={"grid_row_action"}>
                    {record.data_status == "1" ? <RowEnableAction data={record}/> : <RowDisableAction data={record}/>}
                    <arco.Button type={"text"} style={{color: '#666'}} onClick={() => {
                        PageAction.openInfoForm(record);
                    }}>查看
                    </arco.Button>
                </div>

            },
        },
    ];
    return (
        <div className={"grid_table"}>
            <arco.Table stripe columns={tableColumnConfig} data={PageState.tableData} pagination={false}/>
        </div>
    )
}
const GridPage = () => {
    return (
        <div className={"grid_page"}>
            <arco.Pagination total={PageState.pageData.total} showTotal showJumper sizeCanChange
                             onChange={(pageNum, pageSize) => {
                                 PageState.setPageData({...PageState.pageData, pageNum: pageNum, pageSize: pageSize});
                             }}/>
        </div>
    )
}
