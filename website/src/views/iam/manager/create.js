import {
    Button,
    Space,
    Drawer,
    Table,
} from 'antd';
import Invoke from '../../../apis/kuth';
import { useSearchParams, Link } from "react-router-dom";
import { useState, useEffect } from 'react';

const CreateBindingDrawer = (props) => {
    const {
        open,
        setOpen,
        setLoading,
        selectedTab,
    } = props;
    const [records, setRecords] = useState([]);
    const [createLoading, setCreateLoading] = useState(false);
    const [selectedRowKeys, setSelectedRowKeys] = useState([]);


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
    const [searchParams, setSearchParams] = useSearchParams();
    const limit = searchParams.get('limit') || 20;
    const offset = searchParams.get('offset') || 0;
    const sort = searchParams.get('sort') || 'created_at desc';
    const groupID = searchParams.get('group');

    useEffect(() => {
        if (createLoading) {
            const users = () => {
                Invoke("/v1/users?&limit=" +
                    limit + "&offset=" + offset + "&sort=" + sort).then((resp) => {
                        setRecords((resp.data || []));
                        setCreateLoading(false);
                    }).catch((err) => {
                        setCreateLoading(false);
                    });
            };
            const policies = () => {
                Invoke("/v1/policies?&limit=" +
                    limit + "&offset=" + offset + "&sort=" + sort).then((resp) => {
                        setRecords((resp.data || []));
                        setCreateLoading(false);
                    }).catch((err) => {
                        setCreateLoading(false);
                    });
            };
            console.log(selectedTab);
            const f = selectedTab === '1' ? users : policies;
            f();
        }
    }, [createLoading, selectedTab, limit, offset, sort, setSearchParams]);

    useEffect(() => {
        setCreateLoading(true);
    }, [])

    const onClick = (e) => {
        e.preventDefault()
        selectedRowKeys.forEach((id) => {
            Invoke("/v1/bindings", "POST", 201, {
                group_id: groupID,
                bind_type: Number(selectedTab),
                object_id: id,
            }).then(() => {
            }).catch(() => {
            });
        });
        setTimeout(() => {
            setOpen(false);
            setLoading(true);
        }, 200);
    };
    const onChange = (pagination, filters, sorter) => {
        // TODO: handle
        console.log("p", pagination, "f", filters, "s", sorter)
    }
    const columns = [
        {
            title: '名称',
            dataIndex: 'name',
            render: (text, row) => {
                return <Space size="large">
                    <Link to={`/iam/users/${row.id}`}>{text}</Link>
                </Space>
            },
        },
    ]
    return <Drawer
        title="创建绑定"
        placement="right"
        size="large"
        onClose={() => { setOpen(false) }}
        open={open}
        extra={
            <Space>
                <Button onClick={() => { setOpen(false) }}>取消</Button>
                <Button type="primary" onClick={onClick}>确认</Button>
            </Space>
        }
    >
        <Table
            loading={createLoading}
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
    </Drawer>
}

export default CreateBindingDrawer;