import React, { useState } from 'react';
import { Button, Breadcrumb, Layout, Menu, Avatar, Image } from 'antd';
import {
  LaptopOutlined, NotificationOutlined, UserOutlined, MenuFoldOutlined,
  MenuUnfoldOutlined,
} from '@ant-design/icons';
import './App.css';
import Alerts from './components/alerts';
import Login from './pages/login';

const { Header, Content, Footer, Sider } = Layout;

const items1 = ['1', '2', '3', '4'].map((key) => ({
  key,
  label: `nav ${key}`,
}));

const items2 = [UserOutlined, LaptopOutlined, NotificationOutlined].map((icon, index) => {
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

const App = () => {
  const [collapsed, setCollapsed] = useState(false);
  const toggleCollapsed = () => {
    setCollapsed(!collapsed);
  };
  return (<Layout>
    <Header className="layout-header">
      <div className="layout-header-logo" >Kuth</div>
      <Layout className="layout-header-content">
        <Menu
          theme="dark"
          mode="horizontal"
          defaultSelectedKeys={['2']}
          items={items1}
        />
        <User className="user" userName="crochee" />
      </Layout>
    </Header>
    <Layout className="layout-body">
      <Sider className="layout-sider" trigger={null} collapsible collapsed={collapsed}>
        {React.createElement(collapsed ? MenuUnfoldOutlined : MenuFoldOutlined, {
          className: 'trigger',
          onClick: toggleCollapsed,
        })}
        <Menu
          mode="inline"
          defaultSelectedKeys={['1']}
          items={items2}
        />
      </Sider>
      <Layout style={{ padding: '0 24px 24px' }}>
        <Breadcrumb style={{ margin: '16px 0' }}>
          <Breadcrumb.Item>Home</Breadcrumb.Item>
          <Breadcrumb.Item>List</Breadcrumb.Item>
          <Breadcrumb.Item>App</Breadcrumb.Item>
        </Breadcrumb>
        <Content
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
          <Login />
        </Content>
      </Layout>
    </Layout>
    <Footer
      style={{
        textAlign: 'center',
      }}
    >
      Kuth ©2022 Created by crochee
    </Footer>
  </Layout>
  )
}

export default App;


const User = (props) => {
  const {
    userName,
    imageUrl,
  } = props
  return imageUrl ? <Avatar className="user" src={<Image src={imageUrl} style={{ width: 32 }} />} /> :
    <Avatar className="user" style={{ color: '#f56a00', backgroundColor: '#fde3cf' }}>{userName}</Avatar>
}