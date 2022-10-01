import { Layout, Menu } from 'antd';
import { Outlet, Link, useLocation } from "react-router-dom";

const items = [
    {
        key: '/iam/users',
        label: (
            <Link to="/iam/users">用户</Link>
        ),
    },
    {
        key: '/iam/groups',
        label: (
            <Link to="/iam/groups">用户组</Link>
        ),
    },
    {
        key: '/iam/secrets',
        label: (
            <Link to="/iam/secrets">密钥</Link>
        ),
    },
    {
        key: '/iam/policys',
        label: (
            <Link to="/iam/policys">策略</Link>
        )
    },
];

const Iam = () => {
    let pathname = [useLocation().pathname];
    return <Layout>
        <Layout.Sider className="layout-sider">
            <p >身份识别与访问管理</p>
            <Menu
                selectable={true}
                defaultSelectedKeys={pathname}
                items={items}
            />
        </Layout.Sider>
        <Outlet />
    </Layout>
}

export default Iam;