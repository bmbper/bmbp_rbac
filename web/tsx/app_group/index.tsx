window.onload = () => {
    const root = ReactDOM.createRoot(document.getElementById("app"));
    root.render(<PageView/>);
}

const PageView = () => {
    return <div className={"bmbp_container"}>
        <TreeGridView>
            <TreeView>
                <TreeViewHeader/>
            </TreeView>
            <arco.Divider type={'vertical'} style={{width: "1px", height: "100%", margin: "0 2px"}}/>
            <GridView/>
        </TreeGridView>
    </div>
}

const TreeGridView = (props: any) => {
    return <div className={"bmbp_page_tree_grid"}>
        {props.children}
    </div>
}
const TreeViewHeader = () => {
    return (
        <div className={"tree_header"}>
            <div className={"tree_title"}>
                应用分组
            </div>
            <div className={"tree_action"}>
                ddd
            </div>
        </div>
    )
}

const TreeView = (props: any) => {
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
