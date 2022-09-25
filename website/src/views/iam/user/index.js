import {
    Layout,
    PageHeader,
    Table,
    Tag,
    Form,
    Button,
    Input,
    Collapse,
    Pagination,
    Divider,
    message,
    Select,
    Space
} from "antd";
import { DownOutlined } from '@ant-design/icons';
import { useState, useEffect } from 'react';
import './index.css';
import { useSearchParams, useLocation, useNavigate } from "react-router-dom";
import { GetUserList } from "../../../apis/kuth/user";

const columns = [
    {
        title: '用户名/ID',
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
                <a>Delete</a>
                <a>
                    <Space>
                        More actions
                        <DownOutlined />
                    </Space>
                </a>
            </Space>
        },
    },
];


const UserList = () => {
    const [loading, setLoading] = useState(false);
    const [deleteUnableState, setDeleteUnableState] = useState(true);
    const [selectDeleteData, setSelectDeleteData] = useState([]);
    const [data, setData] = useState([]);
    const rowSelection = {
        onChange: (selectedRowKeys, selectedRows, info) => {
            if (selectedRows.length === 0) {
                setDeleteUnableState(true);
            } else {
                setDeleteUnableState(false);
            }
            setSelectDeleteData(selectedRows);
            console.log(`selectedRowKeys: ${selectedRowKeys} selectedRows: ${selectedRows} info:${info}`);
        },
        getCheckboxProps: (record) => ({
            disabled: record.admin === "企业管理人员1",
            name: record.name,
        }),
    };
    let [searchParams, setSearchParams] = useSearchParams();
    let limit = searchParams.get('limit') || 20;
    let offset = searchParams.get('offset') || 0;
    useEffect(() => {
        setLoading(true);
        GetUserList(limit, offset).then((resp) => {
            setData(resp.data.map((value, index) => {
                let admin = "普通角色";
                if (value.admin === 2) {
                    admin = "企业管理人员"
                }
                return {
                    key: index,
                    name: value.name + "/" + value.id,
                    admin,
                    desc: value.desc,
                    created_at: value.created_at,
                    updated_at: value.updated_at,
                }
            }));
            setSearchParams({
                limit: resp.limit,
                offset: resp.offset,
            })
        })
        setLoading(false);
    }, [])

    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            ghost={true}
            title="用户"
            extra={[
                <Button key="2" disabled={deleteUnableState}>删除用户</Button>,
                <Button key="1" type="primary">创建用户</Button>
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
        </Layout.Content>
    </Layout>
}


export default UserList;