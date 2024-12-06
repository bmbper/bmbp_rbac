export const PageState: any = {}

export const PageAction: any = {
    init: () => {
        const [treeData, setTreeData] = React.useState<any>([])
        PageState.treeData = treeData
        PageState.setTreeData = setTreeData
    }
}
