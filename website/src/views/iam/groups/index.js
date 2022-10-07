import {
    Layout,
    PageHeader,
    Table,
    Button,
    Space,
    Popover,
} from "antd";
import {
    PlusOutlined,
    MinusOutlined,
    QuestionCircleOutlined,
} from '@ant-design/icons';
import { useState, useEffect } from 'react';
import { useSearchParams, Link } from "react-router-dom";
import Invoke from "../../../apis/kuth";
import Group from "./item";
import CreateGroupDrawer from "./create";

const Groups = () => {
    const [records, setRecords] = useState([]);
    const [loading, setLoading] = useState(false);
    const [open, setOpen] = useState(false);
    const [selectedRowKeys, setSelectedRowKeys] = useState([]);
    // 数据拉取
    const [searchParams, setSearchParams] = useSearchParams();
    const limit = searchParams.get('limit') || 20;
    const offset = searchParams.get('offset') || 0;
    const sort = searchParams.get('sort') || 'created_at desc';
    useEffect(() => {
        if (loading) {
            Invoke("/v1/groups?limit=" + limit + "&offset=" + offset + "&sort=" + sort).then((resp) => {
                setRecords((resp.data || []).map((value) => {
                    return {
                        ...value,
                        admin: value.admin === 2 ? "企业管理人员" : "普通角色",
                    }
                }));
                setSearchParams({
                    limit: resp.limit,
                    offset: resp.offset,
                    sort: sort,
                });
                setSelectedRowKeys([]);
                setLoading(false);
            }).catch((err) => {
                setLoading(false);
            });
        }
    }, [loading, limit, offset, sort, setSearchParams]);

    useEffect(() => {
        setLoading(true);
    }, [])
    const columns = [
        {
            title: '用户组ID',
            dataIndex: 'id',
            render: (text) => {
                return <Space size="large">
                    <Link to={`/iam/groups/${text}`}>{text}</Link>
                </Space>
            },
        },
        {
            title: '用户组名',
            dataIndex: 'name',
        },
        {
            title: '描述',
            dataIndex: 'desc',
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
                            Invoke("/v1/groups/" + row.id, 'DELETE', 204).then(() => {
                                setLoading(true);
                            });
                        }}
                        disabled={row.name === "Administrator"}
                    >Delete</Button>
                    <Button
                        key='3'
                        type="primary"
                    >
                        <Link to={`/iam/groups/managers?group=${row.id}`}>用户授权管理</Link>
                    </Button>
                </Space>
            },
        },
    ];

    const onChange = (pagination, filters, sorter) => {
        // TODO: handle
        console.log("p", pagination, "f", filters, "s", sorter)
    }
    const rowSelection = {
        selectedRowKeys,
        onChange: (selectedRowKeys) => {
            setSelectedRowKeys(selectedRowKeys);
        },
        getCheckboxProps: (record) => ({
            disabled: record.name === "Administrator",
            id: record.id,
        }),
    };

    return <Layout style={{ padding: '0 12px' }}>
        <PageHeader
            title={
                <>
                    <span className="ant-page-header-heading-title">用户组</span>
                    <Popover
                        placement="right"
                        content={<>
                            用户组是具有相同权限的用户的集合。通过用户组，可以<br />把相同权限的用户集中管理，提高权限管理的效率
                        </>}
                    >
                        <Button type="text" icon={<QuestionCircleOutlined />} />
                    </Popover>
                </>
            }
            extra={[
                <Button
                    key="2"
                    type="primary"
                    disabled={(selectedRowKeys.length === 0)}
                    icon={<MinusOutlined />}
                    onClick={() => {
                        selectedRowKeys.forEach((id) => {
                            Invoke("/v1/groups/" + id, 'DELETE', 204).then(() => { });
                        });
                        setTimeout(() => {
                            setLoading(true);
                        }, 200);
                    }}
                >删除用户组</Button>,
                <Button
                    key="1"
                    type="primary"
                    icon={<PlusOutlined />}
                    onClick={() => { setOpen(true) }}
                >创建用户组</Button>
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
                rowKey={"id"}
                columns={columns}
                dataSource={records}
                onChange={onChange}
            />
            <CreateGroupDrawer open={open} setOpen={setOpen} setLoading={setLoading} />
        </Layout.Content>
    </Layout>
}


export default Groups;
export { Group };