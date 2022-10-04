import {
    Layout,
    PageHeader,
    Breadcrumb,
    Descriptions,
    Button,
} from 'antd';
import { useParams, Link } from "react-router-dom";
import Invoke from '../../../apis/kuth';
import { useState, useEffect } from 'react';
import { ReloadOutlined } from '@ant-design/icons';
import EditableCell from '../../../components/editor';

const Secret = () => {
    const [data, setData] = useState({
        id: "",
        user_id: "",
        name: "",
        access_key: "",
        expire: 0,
        created_at: "",
        updated_at: "",
    });
    const [loading, setLoading] = useState(false);
    let params = useParams();
    useEffect(() => {
        if (loading) {
            Invoke("/v1/secrets/" + params.id).then((resp) => {
                setData(resp);
                setLoading(false);
            }).catch(() => {
                setLoading(false);
            });
        }
    }, [loading, params.id]);
    useEffect(() => {
        setLoading(true);
    }, []);
    const onSave = (key, value) => {
        Invoke("/v1/secrets/" + params.id, 'PATCH', 204, { [key]: value }).then(() => {
            setLoading(true);
        })
    };
    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            title={<Breadcrumb>
                <Breadcrumb.Item>
                    <Link to="/iam/secrets">密钥</Link>
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
                title={data.name}
            >
                <Descriptions.Item label="ID">{data.id}</Descriptions.Item>
                <Descriptions.Item label="密钥名">
                    <EditableCell content={data.name} onSave={(value) => {
                        onSave("name", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="AK">{data.access_key}</Descriptions.Item>
                <Descriptions.Item label="过期时间">{data.expire}</Descriptions.Item>
                <Descriptions.Item label="创建时间">{data.created_at}</Descriptions.Item>
                <Descriptions.Item label="更新时间">{data.updated_at}</Descriptions.Item>
            </Descriptions>
        </Layout.Content>
    </Layout>
}

export default Secret;
