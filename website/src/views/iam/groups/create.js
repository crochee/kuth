import {
    Button,
    Space,
    Drawer,
    Form,
    Input,
} from 'antd';
import Invoke from '../../../apis/kuth';

const CreateGroupDrawer = (props) => {
    const {
        open,
        setOpen,
        setLoading,
    } = props
    const [form] = Form.useForm();

    const onClick = (e) => {
        e.preventDefault()
        let hasError = false;
        form.getFieldsError().map((field) => {
            if (field.errors.length !== 0) {
                hasError = true
            }
            return field
        })
        if (hasError) {
            return
        }
        form.submit();
        Invoke("/v1/groups", "POST", 201, {
            name: form.getFieldValue("name"),
            desc: form.getFieldValue("desc"),
        }).then((result) => {
            setOpen(false);
            setLoading(true);
            form.resetFields();
        })
    };
    return <Drawer
        title="创建用户组"
        placement="right"
        size="large"
        onClose={() => { setOpen(false) }}
        open={open}
        extra={
            <Space>
                <Button onClick={() => { setOpen(false) }}>取消</Button>
                <Button type="primary" onClick={onClick}>确认</Button>
            </Space>
        }
    >
        <Form form={form} layout="vertical">
            <Form.Item
                name="name"
                label="用户组名"
                rules={[
                    {
                        required: true,
                        message: 'Please enter group name',
                    },
                ]}
            >
                <Input
                    placeholder="Please enter group name"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
            <Form.Item
                name="desc"
                label="描述"
            >
                <Input
                    placeholder="Please enter description"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
        </Form>
    </Drawer>
}

export default CreateGroupDrawer;