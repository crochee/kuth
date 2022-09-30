import {
    Layout,
    PageHeader,
    Table,
    Button,
    Space,
} from "antd";
import { DownOutlined, PlusOutlined, MinusOutlined } from '@ant-design/icons';
import { useState, useEffect } from 'react';
import { useSearchParams, Link } from "react-router-dom";
import Invoke from "../../../apis/kuth";
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
            return <Space size="large">
                <Button type="primary" onClick={() => {
                    Invoke("/v1/users/" + row.id, 'DELETE', 204).then(() => { });
                }}>Delete</Button>
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
    const [records, setRecords] = useState([]);
    const [loading, setLoading] = useState(true);

    // 数据拉取
    const [searchParams, setSearchParams] = useSearchParams();
    const limit = searchParams.get('limit') || 20;
    const offset = searchParams.get('offset') || 0;
    useEffect(() => {
        if (!loading) {
            return
        }
        setLoading(true);
        Invoke("/v1/users?limit=" + limit + "&offset=" + offset).then((resp) => {
            setRecords((resp.data || []).map((value, index) => {
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
    }, [loading, limit, offset, setSearchParams]);

    const [open, setOpen] = useState(false);

    const [batchDeleteUnableState, setBatchDeleteUnableState] = useState(true);
    const [selectDeleteData, setSelectDeleteData] = useState([]);

    const rowSelection = {
        onChange: (selectedRowKeys, selectedRows, info) => {
            if (selectedRows.length === 0) {
                setBatchDeleteUnableState(true);
            } else {
                setBatchDeleteUnableState(false);
            }
            setSelectDeleteData(selectedRows.map((row) => {
                return row.id
            }));
            console.log(`selectedRowKeys:`, selectedRowKeys, "selectedRows:", selectedRows, `info:`, info);
        },
        getCheckboxProps: (record) => ({
            disabled: record.admin === "企业管理人员",
            id: record.id,
        }),
        onSelectNone: () => {
            console.log("clear")
        }
    };


    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            ghost={true}
            title="用户"
            extra={[
                <Button key="2"
                    disabled={batchDeleteUnableState}
                    icon={<MinusOutlined />}
                    onClick={() => {
                        selectDeleteData.forEach((id) => {
                            Invoke("/v1/users/" + id, 'DELETE', 204).then(() => { });
                        });
                    }}
                >删除用户</Button>,
                <Button key="1"
                    type="primary"
                    icon={<PlusOutlined />}
                    onClick={() => { setOpen(true) }}
                >创建用户</Button>
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
                dataSource={records}
            />
            <CreateUserDrawer open={open} setOpen={setOpen} />
        </Layout.Content>
    </Layout>
}



export default Users;
export { User };