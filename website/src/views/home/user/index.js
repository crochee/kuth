import { Avatar, Image, Dropdown, Space, Menu, Typography } from 'antd';
import { Link } from "react-router-dom";
import Logout from "./logout";

const menu = (
    <Menu
        selectable
        defaultSelectedKeys={['1']}
        items={[
            {
                key: '1',
                label: (
                    <Link to="/iam">身份识别与访问管理</Link>
                ),
            },
            {
                key: '2',
                label: (
                    <Link to="/about">关于</Link>
                ),
            },
            {
                key: '3',
                label: (
                    <a target="_blank" rel="noopener noreferrer" href="https://www.aliyun.com">
                        阿里云
                    </a>
                ),
            },
            {
                key: '4',
                label: (
                    < Logout />
                )
            },
        ]}
    />
);

export const User = (props) => {
    const {
        userName,
        imageUrl,
    } = props
    return imageUrl ? <Avatar className="user" src={<Image src={imageUrl} style={{ width: 32 }} />} /> :
        <Avatar className="user" style={{ color: '#f56a00', backgroundColor: '#fde3cf' }}>{userName}</Avatar>
}

const UserDropdown = (props) => {
    return (
        <Dropdown
            overlay={menu}
            placement="bottom"
            arrow={{
                pointAtCenter: true,
            }}
        >
            <Typography.Link>
                <Space>
                    <User {...props} />
                </Space>
            </Typography.Link>

        </Dropdown>
    )
}

export default UserDropdown;