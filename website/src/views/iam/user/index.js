import {
    Layout,
    PageHeader,
    Table,
    Button,
    Space,
} from "antd";
import { DownOutlined, PlusOutlined } from '@ant-design/icons';
import { useState, useEffect } from 'react';
import { useSearchParams, Link } from "react-router-dom";
import { GetUsers } from "../../../apis/kuth/user";
import { User, CreateUserDrawer } from "./form";

const columns = [
    {
        title: '用户ID',
        dataIndex: 'id',
        render: (text) => {
            return <Space size="large">
                <Link to={`/iam/users/${text}`}>{text}</Link>
            </Space>
        },
    },
    {
        title: '用户名',
        dataIndex: 'name',
    },
    {
        title: '类型',
        dataIndex: 'admin',
    },
    {
        title: '描述',
        dataIndex: 'desc',
    },
    {
        title: '创建时间',
        dataIndex: 'created_at',
        sorter: (a, b) => a.created_at - b.created_at,
    },
    {
        title: '更新时间',
        dataIndex: 'updated_at',
        sorter: (a, b) => a.updated_at - b.updated_at,
    },
    {
        title: '操作',
        key: 'action',
        render: (row) => {
            // console.log(row);
            return <Space size="large">
                <Button type="primary">Delete</Button>
                <Button type="primary">
                    <Space>
                        More actions
                        <DownOutlined />
                    </Space>
                </Button>
            </Space>
        },
    },
];

const Users = () => {
    const [loading, setLoading] = useState(false);
    const [open, setOpen] = useState(false);
    const [batchDeleteUnableState, setBatchDeleteUnableState] = useState(true);
    const [selectDeleteData, setSelectDeleteData] = useState([]);
    const [data, setData] = useState([]);
    const rowSelection = {
        onChange: (selectedRowKeys, selectedRows, info) => {
            if (selectedRows.length === 0) {
                setBatchDeleteUnableState(true);
            } else {
                setBatchDeleteUnableState(false);
            }
            setSelectDeleteData(selectedRows);
            console.log(`selectedRowKeys:`, selectedRowKeys, "selectedRows:", selectedRows, `info:`, info);
        },
        getCheckboxProps: (record) => ({
            disabled: record.admin === "企业管理人员",
            id: record.id,
        }),
    };
    const [searchParams, setSearchParams] = useSearchParams();
    const limit = searchParams.get('limit') || 20;
    const offset = searchParams.get('offset') || 0;
    useEffect(() => {
        setLoading(true);
        GetUsers(limit, offset).then((resp) => {
            setData(resp.data.map((value, index) => {
                let admin = "普通角色";
                if (value.admin === 2) {
                    admin = "企业管理人员"
                }
                return {
                    key: index,
                    ...value,
                    admin,
                }
            }));
            setSearchParams({
                limit: resp.limit,
                offset: resp.offset,
            })
        })
        setLoading(false);
    }, [limit, offset, setSearchParams]);

    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            ghost={true}
            title="用户"
            extra={[
                <Button key="2" disabled={batchDeleteUnableState}>删除用户</Button>,
                <Button key="1" type="primary" icon={<PlusOutlined />} onClick={() => { setOpen(true) }}>创建用户</Button>
            ]}
        >
        </PageHeader>
        <Layout.Content
            className="layout-background"
            style={{
                padding: 24,
                margin: 0,
                minHeight: 280,
            }}
        >
            <Table
                loading={loading}
                size="large"
                showHeader={true}
                rowSelection={rowSelection}
                pagination={{
                    position: ["bottomCenter"],
                }}
                columns={columns}
                dataSource={data}
            />
            <CreateUserDrawer open={open} setOpen={setOpen} />
        </Layout.Content>
    </Layout>
}



export default Users;
export { User };