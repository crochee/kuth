import {
    Button,
    Space,
    Drawer,
    Form,
    Input,
    Layout,
    PageHeader,
    Table,
    Popover,
    Breadcrumb,
    Tabs,
} from 'antd';
import Invoke from '../../../apis/kuth';
import { useSearchParams, Link } from "react-router-dom";
import { useState, useEffect } from 'react';

const CreateManagerDrawer = (props) => {
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
    const id = searchParams.get('group');

    const [form] = Form.useForm();
    const onClick = (e) => {
        e.preventDefault()
        let hasError = false;
        form.getFieldsError().map((field) => {
            if (field.errors.length !== 0) {
                hasError = true
            }
            return field
        })
        if (hasError) {
            return
        }
        form.submit();
        Invoke("/v1/binds", "POST", 201, {
            group_id: id,
            bind_type: selectedTab,
            desc: form.getFieldValue("desc"),
        }).then((result) => {
            setOpen(false);
            setLoading(true);
            form.resetFields();
        })
    };
    const onChange = (pagination, filters, sorter) => {
        // TODO: handle
        console.log("p", pagination, "f", filters, "s", sorter)
    }
    const columns = [
        {
            title: '用户名',
            dataIndex: 'name',
            render: (text, row) => {
                return <Space size="large">
                    <Link to={`/iam/users/${row.id}`}>{text}</Link>
                </Space>
            },
        },
    ]
    return <Drawer
        title="创建用户组"
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
            showHeader={false}
            rowSelection={rowSelection}
            pagination={{
                position: ["bottomCenter"],
            }}
            rowKey={"id"}
            columns={columns}
            dataSource={records}
            onChange={onChange}
        />
        <Form form={form} layout="vertical">
            <Form.Item
                name="name"
                label="用户组名"
                rules={[
                    {
                        required: true,
                        message: 'Please enter group name',
                    },
                ]}
            >
                <Input
                    placeholder="Please enter group name"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
            <Form.Item
                name="desc"
                label="描述"
            >
                <Input
                    placeholder="Please enter description"
                    allowClear
                    maxLength={255}
                />
            </Form.Item>
        </Form>
    </Drawer>
}

export default CreateManagerDrawer;