import { Layout, Menu } from 'antd';
import { Outlet, Link, useLocation } from "react-router-dom";

const items = [
    {
        key: 'users',
        label: (
            <Link to="/iam/users">用户</Link>
        ),
    },
    {
        key: 'groups',
        label: (
            <Link to="/iam/groups">用户组</Link>
        ),
    },
    {
        key: 'secrets',
        label: (
            <Link to="/iam/secrets">密钥</Link>
        ),
    },
    {
        key: 'policies',
        label: (
            <Link to="/iam/policies">策略</Link>
        )
    },
];

const Iam = () => {
    let paths = useLocation().pathname.split('/');
    let selectKey = "users";
    if (paths.length > 2) {
        selectKey = paths[2];
    }
    return <Layout>
        <Layout.Sider className="layout-sider">
            <p >身份识别与访问管理</p>
            <Menu
                selectable={true}
                defaultSelectedKeys={[selectKey]}
                items={items}
            />
        </Layout.Sider>
        <Outlet />
    </Layout>
}

export default Iam;