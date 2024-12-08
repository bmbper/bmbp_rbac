import {PageState} from "./action";

export const AddForm = () => {
    React.useEffect(() => {
        if (PageState.formData) {
            PageState.addFormRef.current?.setFieldsValue(PageState.formData);
        }
    }, [PageState.formData]);
    return <>
        <arco.Form ref={PageState.addFormRef} style={{width: "100%"}} autoComplete='off'>
            <FormFields/>
        </arco.Form>
    </>
}
export const EditForm = () => {
    React.useEffect(() => {
        if (PageState.formData) {
            PageState.editFormRef.current?.setFieldsValue(PageState.formData);
        }
    }, [PageState.formData]);
    return <arco.Form ref={PageState.editFormRef} style={{width: "100%"}} autoComplete='off' disabled>
        <FormFields/>
    </arco.Form>
}
export const DetailForm = () => {
    React.useEffect(() => {
        if (PageState.formData) {
            PageState.detailFormRef.current?.setFieldsValue(PageState.formData);
        }
    }, [PageState.formData]);
    return <arco.Form ref={PageState.detailFormRef} style={{width: "100%"}} autoComplete='off' disabled>
        <FormFields/>
    </arco.Form>
}
export const FormFields = () => {
    return <>
        <arco.Form.Item label='主键' field='data_id' hidden={true}>
            <arco.Input/>
        </arco.Form.Item>
        <arco.Form.Item label='上级编码' field='node_parent_code' hidden={true}>
            <arco.Input/>
        </arco.Form.Item>
        <arco.Form.Item label='上级名称' field='node_parent_naem'>
            <arco.Input readOnly/>
        </arco.Form.Item>
        <arco.Form.Item label='分组名称' field='node_name' rules={[{required: true, message: '请输入分组名称'}, {
            maxLength: 32,
            message: '最多输入 32 个字'
        }]}>
            <arco.Input placeholder={'请输入分组名称'}/>
        </arco.Form.Item>
        <arco.Form.Item label='分组描述' field='node_desc' rules={[{
            maxLength: 128,
            message: '最多输入 128 个字'
        }]}>
            <arco.Input placeholder={'请输入分组描述'}/>
        </arco.Form.Item>
        <arco.Form.Item label='显示排序' field='node_order'>
            <arco.InputNumber placeholder={'请输入显示排序'}/>
        </arco.Form.Item>
    </>
}
export const OwnerForm = () => {
    return <div>EditForm</div>
}

