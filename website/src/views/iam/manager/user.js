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

const ManagerUsers = (props) => {
    const {
        selectedRowKeys,
        setSelectedRowKeys,
        rowSelection,
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
            Invoke("/v1/binds?group_id=" + id + "&limit=" +
                limit + "&offset=" + offset + "&sort=" + sort).then((resp) => {
                    setRecords((resp.data || []));
                    setLoading(false);
                }).catch((err) => {
                    setLoading(false);
                });
        }
    }, [loading, id, limit, offset, sort, setSearchParams]);

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
            title: '类型',
            dataIndex: 'bind_type',
            render: (text) => {
                if (text === 1) {
                    return '角色'
                }
                if (text === 2) {
                    return '策略'
                }
                return text
            },
        },
        {
            title: '目标ID',
            dataIndex: 'object_id',
            render: (text, record) => {
                if (record.bind_type === 1) {
                    return <Space size="large">
                        <Link to={`/iam/users/${text}`}>{text}</Link>
                    </Space>
                }
                if (record.bind_type === 2) {
                    return <Space size="large">
                        <Link to={`/iam/policies/${text}`}>{text}</Link>
                    </Space>
                }
                return text
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
                            // Invoke("/v1/binds/" + row.id, 'DELETE', 204).then(() => {
                            //     setLoading(true);
                            // });
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


export default ManagerUsers;