import React from 'react';
import './index.css';
import { Button, Checkbox, Form, Input } from 'antd';
import { PostTokens } from '../../apis/kuth/auth';
import { LockOutlined, UserOutlined } from '@ant-design/icons';
import { useDispatch } from 'react-redux';
import { GetUser } from "../../apis/kuth/user";
import { UserSetToken, UserSetInfo } from "../../store";
import { useNavigate, useLocation } from "react-router-dom";

const Login = () => {
    const dispatch = useDispatch();
    let location = useLocation();
    let from = location.state?.from?.pathname || "/";
    const navigate = useNavigate();
    const onFinish = (values) => {
        PostTokens(values.username, values.password).then((response) => {
            dispatch(UserSetToken(response.Token));
            GetUser(response.User).then(resp => {
                dispatch(UserSetInfo(resp))
                navigate(from, { replace: true });
            })
        })
    };
    return (
        <Form
            name="normal_login"
            className="login-form"
            initialValues={{
                remember: true,
            }}
            onFinish={onFinish}
        >
            <Form.Item
                name="username"
                rules={[
                    {
                        required: true,
                        message: 'Please input your Username!',
                    },
                ]}
            >
                <Input prefix={<UserOutlined className="site-form-item-icon" />} placeholder="Username" />
            </Form.Item>
            <Form.Item
                name="password"
                rules={[
                    {
                        required: true,
                        message: 'Please input your Password!',
                    },
                ]}
            >
                <Input
                    prefix={<LockOutlined className="site-form-item-icon" />}
                    type="password"
                    placeholder="Password"
                />
            </Form.Item>
            <Form.Item>
                <Form.Item name="remember" valuePropName="checked" noStyle>
                    <Checkbox>Remember me</Checkbox>
                </Form.Item>
                <a className="login-form-forgot" href="/">
                    Forgot password
                </a>
            </Form.Item>
            <Form.Item>
                <Button type="primary" htmlType="submit" className="login-form-button">
                    Login
                </Button>
                Or <a href="/">register now!</a>
            </Form.Item>
        </Form>
    );
};

export default Login;