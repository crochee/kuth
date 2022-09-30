import React, { useState, useEffect } from 'react';
import { Layout, Menu, message } from 'antd';
import {
    LaptopOutlined, NotificationOutlined, UserOutlined, MenuFoldOutlined,
    MenuUnfoldOutlined,
} from '@ant-design/icons';
import './index.css';
import UserDropdown from './user';
import { Outlet, Link, useLocation, useNavigate } from "react-router-dom";
import { useSelector, useDispatch } from 'react-redux';
import Invoke, { Request } from "../../apis/kuth";
import { UserSetInfo, UserClear } from "../../store";
import Search from "./search";

const headerItems = [
    {
        key: 'invoices',
        label: (
            <Link to="/invoices">Invoices</Link>
        ),
    },
    {
        key: 'expenses',
        label: (
            <Link to="/expenses">Expenses</Link>
        ),
    },
];

const siderItems = [
    {
        label: 'User',
        key: 'user',
        icon: <UserOutlined />,
        children: [
            {
                key: '1',
                label: "sub1"
            },
            {
                key: '2',
                label: "sub2"
            }
        ]
    },
    {
        label: 'Laptop',
        key: 'laptop',
        icon: <LaptopOutlined />,
    },
    {
        label: 'Notify',
        key: 'notify',
        icon: <NotificationOutlined />,
    },
];

const Home = () => {
    const [collapsed, setCollapsed] = useState(true);
    const toggleCollapsed = () => {
        setCollapsed(!collapsed);
    };
    const userStore = useSelector((state) => state.user);
    return <Layout>
        <Layout.Header className="layout-header">
            <Link to="/" className="layout-header-logo">Kuth</Link>
            <Layout className="layout-header-content">
                <Menu
                    theme="dark"
                    mode="horizontal"
                    defaultSelectedKeys={['invoices']}
                    items={headerItems}
                />
                <Layout className="layout-header-suffix">
                    <Search />
                    <UserDropdown userName={userStore.name} imageUrl={userStore.image} />
                </Layout>
            </Layout>
            <CheckAuth />
        </Layout.Header>
        <Layout className="layout-body">
            <Layout.Sider className="layout-sider" trigger={null} collapsible collapsed={collapsed}>
                {React.createElement(collapsed ? MenuUnfoldOutlined : MenuFoldOutlined, {
                    className: 'trigger',
                    onClick: toggleCollapsed,
                })}
                <Menu
                    mode="inline"
                    defaultSelectedKeys={['user']}
                    items={siderItems}
                />
            </Layout.Sider>
            <Layout style={{ padding: '0 24px' }}>
                <Layout.Content
                    className="layout-background"
                    style={{
                        padding: 24,
                        margin: 0,
                        minHeight: 280,
                    }}
                >
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

const CheckAuth = () => {
    const userStore = useSelector((state) => state.user);
    const dispatch = useDispatch();
    let location = useLocation();
    const navigate = useNavigate();
    useEffect(() => {
        if (!userStore.token) {
            navigate("/login", { state: { from: location }, replace: true });
            return
        }
        if (userStore.id) {
            return
        }
        Request("/v1/auth?action=post&path=/v1/auth", "POST").then((response) => {
            if (response.status === 200) {
                response.json().then((resp) => {
                    Invoke("/v1/users/" + resp.user_id).then(resp => {
                        dispatch(UserSetInfo(resp))
                    })
                })
                return
            }
            response.json().then((resp) => {
                dispatch(UserClear());
                navigate("/login", { state: { from: location }, replace: true });
                if (resp.message.startsWith("kuth.404")) {
                    message.error("会话过期,请重新登陆", 5);
                    return
                }
                message.error(resp.message, 5);
            })
        }).catch((content) => {
            dispatch(UserClear());
            navigate("/login", { state: { from: location }, replace: true });
            message.error(content, 5)
        })
    })
    return (<></>)
}