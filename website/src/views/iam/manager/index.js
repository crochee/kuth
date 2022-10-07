import {
    Layout,
    PageHeader,
    Table,
    Button,
    Space,
    Popover,
    Breadcrumb,
    Tabs,
} from "antd";
import {
    MinusOutlined,
    QuestionCircleOutlined,
    PlusOutlined,
} from '@ant-design/icons';
import { useState, useEffect } from 'react';
import { useSearchParams, Link } from "react-router-dom";
import Invoke from "../../../apis/kuth";
import CreateManagerDrawer from './create';
import ManagerUsers from './user';


const Managers = () => {
    const [records, setRecords] = useState([]);
    const [loading, setLoading] = useState(false);
    const [selectedTab, setSelectedTab] = useState('1');
    const [open, setOpen] = useState(false);
    const [selectedRowKeys, setSelectedRowKeys] = useState([]);

    const rowSelection = {
        selectedRowKeys,
        onChange: (selectedRowKeys) => {
            setSelectedRowKeys(selectedRowKeys);
        },
        getCheckboxProps: (record) => ({
            disabled: record.id === "145755342888836871",
            id: record.id,
        }),
    };

    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            title={
                <Breadcrumb>
                    <Breadcrumb.Item>
                        <Link to="/iam/groups">用户组</Link>
                    </Breadcrumb.Item>
                    <Breadcrumb.Item>
                        用户授权管理
                    </Breadcrumb.Item>
                </Breadcrumb>
            }
            extra={[
                <Popover placement="left" content="用户授权管理表示基于用户组绑定用户或策略进行批量授权" >
                    <Button type="text" icon={<QuestionCircleOutlined />} />
                </Popover>,
                <Button
                    key="unbind"
                    type="primary"
                    disabled={(selectedRowKeys.length === 0)}
                    icon={<MinusOutlined />}
                    onClick={() => {
                        selectedRowKeys.forEach((id) => {
                            Invoke("/v1/binds/" + id, 'DELETE', 204).then(() => { });
                        });
                        setTimeout(() => {
                            setLoading(true);
                        }, 200);
                    }}
                >解绑</Button>,
                <Button
                    key="bind"
                    type="primary"
                    icon={<PlusOutlined />}
                    onClick={() => { setOpen(true) }}
                >绑定</Button>
            ]}
        />
        <Layout.Content
            className="layout-background"
            style={{
                padding: 24,
                margin: 0,
                minHeight: 280,
            }}
        >
            <Tabs
                defaultActiveKey="1"
                tabPosition="left"
                type="card"
                onChange={(key) => {
                    console.log(key);
                    setSelectedTab(key);
                    setSelectedRowKeys([]);
                }}
                items={[
                    {
                        label: `用户`,
                        key: "1",
                        children: (<ManagerUsers
                            selectedRowKeys={selectedRowKeys}
                            rowSelection={rowSelection}
                            setSelectedRowKeys={setSelectedRowKeys}
                        />),
                    },
                    {
                        label: `策略`,
                        key: "2",
                        children: `Content of Tab Pane 2`,
                    },
                ]}
            />
            <CreateManagerDrawer
                open={open}
                setOpen={setOpen}
                setLoading={setLoading}
                selectedTab={selectedTab}
            />
        </Layout.Content>
    </Layout >
}


export default Managers;