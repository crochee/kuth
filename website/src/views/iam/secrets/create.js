import {
    Button,
    Space,
    Drawer,
    Form,
    Input,
    InputNumber,
} from 'antd';
import Invoke from '../../../apis/kuth';

const CreateSecretDrawer = (props) => {
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
        Invoke("/v1/secrets", "POST", 201, {
            name: form.getFieldValue("name"),
            expire: form.getFieldValue("expire"),
        }).then((result) => {
            setOpen(false);
            setLoading(true);
            form.resetFields();
        })
    };
    return <Drawer
        title="创建密钥"
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
                label="密钥名"
                rules={[
                    {
                        required: true,
                        message: 'Please enter secret name',
                    },
                ]}
            >
                <Input
                    placeholder="Please enter secret name"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
            <Form.Item
                name="expire"
                label="过期时间"
            >
                <InputNumber
                    placeholder="Please enter expire"
                    min={1}
                />
            </Form.Item>
        </Form>
    </Drawer>
}

export default CreateSecretDrawer;