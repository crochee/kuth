import {
    UserOutlined,
    EyeInvisibleOutlined,
    EyeTwoTone,
    LockOutlined,
} from '@ant-design/icons';
import {
    Button,
    Space,
    Drawer,
    Form,
    Input,
} from 'antd';
import Invoke from '../../../apis/kuth';

const CreatePolicyDrawer = (props) => {
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
        Invoke("/v1/policies", "POST", 201, {
            name: form.getFieldValue("name"),
            password: form.getFieldValue("password"),
            desc: form.getFieldValue("desc"),
        }).then((result) => {
            setOpen(false);
            setLoading(true);
            form.resetFields();
        })
    };
    return <Drawer
        title="创建策略"
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
                label="用户名"
                rules={[
                    {
                        required: true,
                        message: 'Please enter user name',
                    },
                ]}
            >
                <Input
                    placeholder="Please enter user name"
                    prefix={<UserOutlined />}
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
            <Form.Item
                name="password"
                label="密码"
                rules={[
                    {
                        required: true,
                        message: 'Please enter password',
                    },
                ]}
            >
                <Input.Password
                    placeholder="Please enter password"
                    allowClear
                    maxLength={255}
                    prefix={<LockOutlined />}
                    iconRender={(visible) => (visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />)}
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

export default CreatePolicyDrawer;