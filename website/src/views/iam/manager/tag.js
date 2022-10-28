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
import CreateBindingDrawer from './create';

const TagDesc = (props) => {
    const {
        selectedRowKeys,
        setSelectedRowKeys,
        rowSelection,
        selectedTab,
    } = props;
    const [records, setRecords] = useState([]);
    const [loading, setLoading] = useState(false);
    const [open, setOpen] = useState(false);
    // 数据拉取
    const [searchParams, setSearchParams] = useSearchParams();
    const limit = searchParams.get('limit') || 20;
    const offset = searchParams.get('offset') || 0;
    const sort = searchParams.get('sort') || 'created_at desc';
    const id = searchParams.get('group');
    useEffect(() => {
        if (loading) {
            Invoke("/v1/bindings?bind_type=" + selectedTab + "&group_id=" + id + "&limit=" +
                limit + "&offset=" + offset + "&sort=" + sort).then((resp) => {
                    setRecords((resp.data || []));
                    setLoading(false);
                }).catch((err) => {
                    setLoading(false);
                });
        }
    }, [loading, selectedTab, id, limit, offset, sort, setSearchParams]);

    useEffect(() => {
        setLoading(true);
    }, [])
    const columns = [
        {
            title: 'ID',
            dataIndex: 'id',
        },
        {
            title: '用户组ID',
            dataIndex: 'group_id',
        },
        {
            title: selectedTab == '1' ? '用户ID' : '策略ID',
            dataIndex: 'object_id',
            render: (text) => {
                return selectedTab == '1' ? <Space size="large">
                    <Link to={`/iam/users/${text}`}>{text}</Link>
                </Space> : <Space size="large">
                    <Link to={`/iam/policies/${text}`}>{text}</Link>
                </Space>
            },
        },
        {
            title: '创建时间',
            dataIndex: 'created_at',
            sorter: {
                compare: (a, b) => a.created_at.localeCompare(b.created_at),
                multiple: 2,
            }
        },
        {
            title: '更新时间',
            dataIndex: 'updated_at',
            sorter: {
                compare: (a, b) => a.updated_at.localeCompare(b.updated_at),
                multiple: 1,
            }
        },
        {
            title: '操作',
            key: 'action',
            render: (row) => {
                return <Space size="large">
                    <Button
                        key='1'
                        type="primary"
                        onClick={(e) => {
                            e.preventDefault();
                            Invoke("/v1/bindings/" + row.id, 'DELETE', 204).then(() => {
                                setLoading(true);
                            });
                        }}
                        disabled={row.name === "Administrator"}
                    >解绑</Button>
                </Space>
            },
        },
    ];

    const onChange = (pagination, filters, sorter) => {
        // TODO: handle
        console.log("p", pagination, "f", filters, "s", sorter)
    }
    return <Table
        loading={loading}
        size="large"
        showHeader={true}
        rowSelection={rowSelection}
        pagination={{
            position: ["bottomCenter"],
        }}
        rowKey={"id"}
        columns={columns}
        dataSource={records}
        onChange={onChange}
    />
}


export default TagDesc;