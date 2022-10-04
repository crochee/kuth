import {
    Button,
    Space,
    Drawer,
    Form,
    Input,
} from 'antd';
import Invoke from '../../../apis/kuth';
import JsonEditor from '../../../components/jsoneditor';
import { useState } from 'react';

const CreatePolicyDrawer = (props) => {
    const {
        open,
        setOpen,
        setLoading,
    } = props
    const [jsonValue, setJsonValue] = useState(null);
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
            desc: form.getFieldValue("desc"),
            policy_type: 1,
            ...jsonValue,
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
                label="策略名"
                rules={[
                    {
                        required: true,
                        message: 'Please enter policy name',
                    },
                ]}
            >
                <Input
                    placeholder="Please enter policy name"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
            <Form.Item
                name="desc"
                label="描述"
                rules={[
                    {
                        required: true,
                        message: 'Please enter desc',
                    },
                ]}
            >
                <Input
                    placeholder="Please enter desc"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
        </Form>
        <JsonEditor
            value={jsonValue}
            onChange={setJsonValue}
            mode="code"
        />
    </Drawer>
}

export default CreatePolicyDrawer;