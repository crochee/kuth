import {
    ReloadOutlined,
} from '@ant-design/icons';
import {
    Layout,
    PageHeader,
    Breadcrumb,
    Button,
    Descriptions,
} from 'antd';
import { useParams, Link } from "react-router-dom";
import Invoke from '../../../apis/kuth';
import { useState, useEffect } from 'react';
import EditableCell from '../../../components/editor';

const Group = () => {
    const [data, setData] = useState({
        id: "",
        account_id: "",
        name: "",
        desc: "",
        created_at: "",
        updated_at: "",
    });
    const [loading, setLoading] = useState(false);
    let params = useParams();
    useEffect(() => {
        if (loading) {
            setLoading(true);
            Invoke("/v1/groups/" + params.id).then((resp) => {
                setData(resp);
                setLoading(false);
            }).catch((err) => {
                setLoading(false);
            });
        }
    }, [loading, params.id]);
    useEffect(() => {
        setLoading(true);
    }, []);
    const onSave = (key, value) => {
        Invoke("/v1/groups/" + params.id, 'PATCH', 204, { [key]: value }).then(() => {
            setLoading(true);
        })
    };
    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            title={<Breadcrumb>
                <Breadcrumb.Item>
                    <Link to="/iam/groups">用户组</Link>
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
                <Descriptions.Item label="用户组名">
                    <EditableCell content={data.name} onSave={(value) => {
                        onSave("name", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="描述">
                    <EditableCell content={data.desc} onSave={(value) => {
                        onSave("desc", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="创建时间">{data.created_at}</Descriptions.Item>
                <Descriptions.Item label="更新时间">{data.updated_at}</Descriptions.Item>
            </Descriptions>
        </Layout.Content>
    </Layout>
}

export default Group;