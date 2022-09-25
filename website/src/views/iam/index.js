import { Layout, Menu } from 'antd';
import React, { useState } from 'react';
import { Outlet, Link, useLocation, useNavigate } from "react-router-dom";

const iamItems = [
    {
        key: '1',
        label: (
            <Link to="/iam/users">用户</Link>
        ),
    },
    {
        key: '2',
        label: (
            <Link to="/iam/groups">用户组</Link>
        ),
    },
    {
        key: '3',
        label: (
            <Link to="/iam/secrets">密钥</Link>
        ),
    },
    {
        key: '4',
        label: (
            <Link to="/iam/policys">策略</Link>
        )
    },
];

const Iam = () => {
    return <Layout>
        <Layout.Sider className="layout-sider">
            <p >身份识别与访问管理</p>
            <Menu
                selectable
                defaultSelectedKeys={['1']}
                items={iamItems}
            />
        </Layout.Sider>
        <Outlet />
    </Layout>
}

export default Iam;