import {
    EditOutlined,
    UserOutlined,
    EllipsisOutlined,
    SettingOutlined,
    EyeInvisibleOutlined,
    EyeTwoTone,
    LockOutlined,
} from '@ant-design/icons';
import {
    Layout,
    PageHeader,
    Avatar,
    Card,
    Breadcrumb,
    Button,
    Space,
    Drawer,
    Form,
    Input,
} from 'antd';
import { useParams, Link } from "react-router-dom";
import { GetUser, CreateUser } from '../../../apis/kuth/user';
import { useState, useEffect } from 'react';

export const User = () => {
    const [data, setData] = useState({
        id: "",
        account_id: "",
        admin: 0,
        name: "",
        desc: "",
        email: "",
        check: 0,
        sex: "",
        image: "",
        created_at: "",
        updated_at: "",
    });
    let params = useParams();
    useEffect(() => {
        GetUser(params.id).then((resp) => {
            setData(resp)
        })
    }, [params.id])
    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader>
            <Breadcrumb>
                <Breadcrumb.Item>
                    <Link to="/iam/users">用户</Link>
                </Breadcrumb.Item>
                <Breadcrumb.Item>{data.name}</Breadcrumb.Item>
            </Breadcrumb>
        </PageHeader>
        <Layout.Content
            className="layout-background"
            style={{
                padding: 24,
                margin: 0,
                minHeight: 280,
            }}
        >
            <Card
                style={{
                    width: 300,
                }}
                cover={
                    <img
                        alt="example"
                        src="https://gw.alipayobjects.com/zos/rmsportal/JiqGstEfoWAOHiTxclqi.png"
                    />
                }
                actions={[
                    <SettingOutlined key="setting" />,
                    <EditOutlined key="edit" />,
                    <EllipsisOutlined key="ellipsis" />,
                ]}
            >
                <Card.Meta
                    avatar={<Avatar src="https://joeschmoe.io/api/v1/random" />}
                    title={data.name}
                    description={data.desc}
                />
            </Card>
        </Layout.Content>
    </Layout>
}

export const CreateUserDrawer = (props) => {
    const {
        open,
        setOpen,
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
        setOpen(false);
        form.submit();
        CreateUser(
            form.getFieldValue("name"),
            form.getFieldValue("password"),
            form.getFieldValue("desc")).then((result) => {
                console.log(result.id);
            })
        form.resetFields();
    };
    return <Drawer
        title="创建用户"
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