import React, { useState } from 'react';
import { Layout, Menu } from 'antd';
import {
    LaptopOutlined, NotificationOutlined, UserOutlined, MenuFoldOutlined,
    MenuUnfoldOutlined,
} from '@ant-design/icons';
import './index.css';
import UserDropdown from './user';
import { Outlet, Link, Navigate, useLocation } from "react-router-dom";
import { useSelector, useDispatch } from 'react-redux';
import { GetUserInfo } from "../../apis/kuth/user";
import { VerifyToken } from "../../apis/kuth/auth";
import { UserSetInfo, UserClear } from "../../store";

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

const Home = () => {
    const [collapsed, setCollapsed] = useState(false);
    const toggleCollapsed = () => {
        setCollapsed(!collapsed);
    };
    const userStore = useSelector((state) => state.user);
    const dispatch = useDispatch();
    let location = useLocation();
    if (!userStore.token) {
        return <Navigate to="/login" state={{ from: location }} />;
    }
    if (!userStore.id) {
        VerifyToken().then(response => {
            if (response.decision === 'Allow') {
                GetUserInfo(response.user_id).then(resp => {
                    dispatch(UserSetInfo(resp))
                })
                return
            }
            dispatch(UserClear());
        })
    }
    return <Layout>
        <Layout.Header className="layout-header">
            <Link to="/" className="layout-header-logo">Kuth</Link>
            <Layout className="layout-header-content">
                <Menu
                    theme="dark"
                    mode="horizontal"
                    defaultSelectedKeys={['2']}
                    items={headerItems}
                />
                <UserDropdown userName={userStore.name} imageUrl={userStore.image} />
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
                <Layout.Content
                    className="layout-background"
                    style={{
                        padding: 24,
                        margin: 0,
                        minHeight: 280,
                    }}
                >
                    <p>Content</p>
                    <Link to="/invoices">Invoices</Link> |{" "}
                    <Link to="/expenses">Expenses</Link>
                    <Outlet />
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
    </Layout >
}

export default Home;