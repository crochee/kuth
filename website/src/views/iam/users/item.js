import {
    Layout,
    PageHeader,
    Breadcrumb,
    Descriptions,
    Image,
    Button,
    message,
} from 'antd';
import { useParams, Link } from "react-router-dom";
import Invoke from '../../../apis/kuth';
import { useState, useEffect } from 'react';
import { ReloadOutlined } from '@ant-design/icons';
import EditableCell from '../../../components/editor';

const User = () => {
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
    const [loading, setLoading] = useState(true);
    let params = useParams();
    useEffect(() => {
        if (loading) {
            setLoading(true);
            Invoke("/v1/users/" + params.id).then((resp) => {
                setData(resp)
            })
            setLoading(false);
        }
    }, [loading, params.id])
    const onSave = (key, value) => {
        Invoke("/v1/users/" + params.id, 'PATCH', 204, { [key]: value }).then(() => {
            setLoading(true);
        })
    };
    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            title={<Breadcrumb>
                <Breadcrumb.Item>
                    <Link to="/iam/users">用户</Link>
                </Breadcrumb.Item>
                <Breadcrumb.Item>{data.name}</Breadcrumb.Item>
            </Breadcrumb>}
            extra={[
                <Button key="1"
                    type="primary"
                    icon={<ReloadOutlined />}
                    onClick={() => { setLoading(true) }}
                >刷新</Button>
            ]}
        />
        <Layout.Content
            className="layout-background"
            style={{
                padding: 24,
                minHeight: 280,
            }}
        >
            <Descriptions
                title={data.image ? <Image src={data.image} /> : data.name}
            >
                <Descriptions.Item label="ID">{data.id}</Descriptions.Item>
                <Descriptions.Item label="用户名">
                    <EditableCell content={data.name} onSave={(value) => {
                        onSave("name", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="性别">
                    <EditableCell content={zhCNSex(data.sex)} onSave={(value) => {
                        const target = enSex(value);
                        if (target) {
                            if (target === 'none') {
                                return
                            }
                            onSave("sex", target);
                            return
                        }
                        message.error("sex has invalid value", 5);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="类型">{data.admin === 2 ? "企业管理人员" : "普通角色"}</Descriptions.Item>
                <Descriptions.Item label="认证">{data.check === 1 ? '否' : '是'}</Descriptions.Item>
                <Descriptions.Item label="描述">
                    <EditableCell content={data.desc} onSave={(value) => {
                        onSave("desc", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="邮箱">
                    <EditableCell content={data.email} onSave={(value) => {
                        onSave("email", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="头像">
                    <EditableCell content={data.image} onSave={(value) => {
                        onSave("image", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="创建时间">{data.created_at}</Descriptions.Item>
                <Descriptions.Item label="更新时间">{data.updated_at}</Descriptions.Item>
            </Descriptions>
        </Layout.Content>
    </Layout>
}

export default User;

const enSex = (value) => {
    console.log(value);
    if (value === '无') {
        return 'none'
    }
    if (value === '男') {
        value = 'male'
    }
    if (value === '女') {
        value = 'female'
    }
    if (value === 'male' || value === 'female') {
        return value;
    }
    return undefined;
}

const zhCNSex = (value) => {
    if (!value) {
        return '无'
    }
    if (value === 'male') {
        value = '男'
    }
    if (value === 'female') {
        value = '女'
    }
    if (value === '男' || value === '女') {
        return value;
    }
    return '无'
}