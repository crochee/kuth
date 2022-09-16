import React, { useState } from 'react';
import { Button, Breadcrumb, Layout, Menu } from 'antd';
import {
    LaptopOutlined, NotificationOutlined, UserOutlined, MenuFoldOutlined,
    MenuUnfoldOutlined,
} from '@ant-design/icons';
import './index.css';
import User from './user';
import Alerts from '../../components/alerts';

const headerItems = ['1', '2', '3', '4'].map((key) => ({
    key,
    label: `nav ${key}`,
}));

const siderItems = [UserOutlined, LaptopOutlined, NotificationOutlined].map((icon, index) => {
    const key = String(index + 1);
    return {
        key: `sub${key}`,
        icon: React.createElement(icon),
        label: `subnav ${key}`,
        children: new Array(4).fill(null).map((_, j) => {
            const subKey = index * 4 + j + 1;
            return {
                key: subKey,
                label: `option${subKey}`,
            };
        }),
    };
});

export default () => {
    const [collapsed, setCollapsed] = useState(false);
    const toggleCollapsed = () => {
        setCollapsed(!collapsed);
    };
    return <Layout>
        <Layout.Header className="layout-header">
            <div className="layout-header-logo" >Kuth</div>
            <Layout className="layout-header-content">
                <Menu
                    theme="dark"
                    mode="horizontal"
                    defaultSelectedKeys={['2']}
                    items={headerItems}
                />
                <User userName="crochee" />
            </Layout>
        </Layout.Header>
        <Layout className="layout-body">
            <Layout.Sider className="layout-sider" trigger={null} collapsible collapsed={collapsed}>
                {React.createElement(collapsed ? MenuUnfoldOutlined : MenuFoldOutlined, {
                    className: 'trigger',
                    onClick: toggleCollapsed,
                })}
                <Menu
                    mode="inline"
                    defaultSelectedKeys={['1']}
                    items={siderItems}
                />
            </Layout.Sider>
            <Layout style={{ padding: '0 24px 24px' }}>
                <Breadcrumb style={{ margin: '16px 0' }}>
                    <Breadcrumb.Item>Home</Breadcrumb.Item>
                    <Breadcrumb.Item>List</Breadcrumb.Item>
                    <Breadcrumb.Item>App</Breadcrumb.Item>
                </Breadcrumb>
                <Layout.Content
                    className="layout-background"
                    style={{
                        padding: 24,
                        margin: 0,
                        minHeight: 280,
                    }}
                >
                    <Button type="primary">Button</Button>
                    <Alerts message="test" type="error" />
                    <p>Content</p>
                </Layout.Content>
            </Layout>
        </Layout>
        <Layout.Footer
            style={{
                textAlign: 'center',
            }}
        >
            Kuth ©2022 Created by crochee
        </Layout.Footer>
    </Layout>
}