import {
    Layout,
    PageHeader,
    Breadcrumb,
    Descriptions,
    Image,
    Button,
    message,
    Dropdown,
    Menu,
} from 'antd';
import { useParams, Link } from "react-router-dom";
import Invoke from '../../../apis/kuth';
import { useState, useEffect } from 'react';
import { ReloadOutlined, DownOutlined } from '@ant-design/icons';
import EditableCell from '../../../components/editor';
import JsonEditor from '../../../components/jsoneditor';




const Policy = () => {
    const [data, setData] = useState({
        id: "",
        name: "",
        desc: "",
        policy_type: 0,
        data: {
            version: "",
            effect: "",
            action: [],
            collection: [],
            resources: [],
            subjects: [],
        },
        created_at: "",
        updated_at: "",
    });
    const [loading, setLoading] = useState(false);
    const [mode, setMode] = useState("view");
    const [jsonValue, setJsonValue] = useState(null);
    let params = useParams();
    useEffect(() => {
        if (loading) {
            Invoke("/v1/policies/" + params.id).then((resp) => {
                const tempData = {
                    version: resp.version,
                    effect: resp.effect,
                    action: resp.action,
                    collection: resp.collection,
                    resources: resp.resources,
                    subjects: resp.subjects,
                }
                setData({
                    id: resp.id,
                    name: resp.name,
                    desc: resp.desc,
                    policy_type: resp.policy_type,
                    data: tempData,
                    created_at: resp.created_at,
                    updated_at: resp.updated_at,
                });
                setJsonValue(tempData);
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
        Invoke("/v1/policies/" + params.id, 'PATCH', 204, { [key]: value }).then(() => {
            setLoading(true);
        })
    };
    const multipleSave = () => {
        Invoke("/v1/policies/" + params.id, 'PATCH', 204, jsonValue).then(() => {
            setLoading(true);
        })
    }

    const jsonMenu = (
        <Menu
            onClick={({ key }) => {
                setMode(key);
            }}
            items={[
                {
                    label: '查看',
                    key: 'view',
                },
                {
                    label: '编辑',
                    key: 'code',
                },
                {
                    label: '文本',
                    key: 'text',
                },
                {
                    label: '树',
                    key: 'tree',
                },
                {
                    label: '表单',
                    key: 'form',
                },
                {
                    label: '预览',
                    key: 'preview',
                },
            ]}
        />
    );
    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            title={<Breadcrumb>
                <Breadcrumb.Item>
                    <Link to="/iam/policies">策略</Link>
                </Breadcrumb.Item>
                <Breadcrumb.Item>{data.name}</Breadcrumb.Item>
            </Breadcrumb>}
            extra={[
                <Dropdown.Button
                    key='2'
                    type="primary"
                    icon={<DownOutlined />}
                    overlay={jsonMenu}
                    onClick={(e) => {
                        e.preventDefault();
                        multipleSave();
                    }}
                >
                    确认
                </Dropdown.Button>,
                <Button
                    key='1'
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
                <Descriptions.Item label="名称">
                    <EditableCell content={data.name} onSave={(value) => {
                        onSave("name", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="描述">
                    <EditableCell content={data.desc} onSave={(value) => {
                        onSave("desc", value);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="类型">
                    <EditableCell content={data.policy_type === 2 ? "系统策略" : "自定义策略"} onSave={(value) => {
                        if (value) {
                            if (value === '系统策略') {
                                onSave("policy_type", 2);
                                return
                            }
                            if (value === '自定义策略') {
                                onSave("policy_type", 1);
                                return
                            }
                        }
                        message.error("sex has invalid value", 5);
                    }} />
                </Descriptions.Item>
                <Descriptions.Item label="创建时间">{data.created_at}</Descriptions.Item>
                <Descriptions.Item label="更新时间">{data.updated_at}</Descriptions.Item>
            </Descriptions>
            <JsonEditor
                value={data.data}
                onChange={setJsonValue}
                mode={mode}
            />
        </Layout.Content>
    </Layout>
}

export default Policy;