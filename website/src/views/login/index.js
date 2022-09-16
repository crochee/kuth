import { useState, useEffect } from 'react';
import './index.css';
import { Button, Checkbox, Form, Input, message } from 'antd';
import Alerts from '../../components/alerts';
import { PostTokens } from '../../apis/kuth/auth';

const Login = (props) => {
    const [userInfo, setInfo] = useState({
        userName: "",
        password: "",
    });
    const onFinish = (values) => {
        console.log('Success:', values.username, values.password);
        setInfo({
            userName: values.username,
            password: values.password,
        })
    };
    useEffect(() => {
        PostTokens(userInfo.userName || "", userInfo.password || "").then((res) => {
            console.log(res)
        })
    }, []);
    let failedErrorInfo;
    const onFinishFailed = (errorInfo) => {
        failedErrorInfo = errorInfo;
    };

    return (
        <div>
            <Form
                name="basic"
                labelCol={{
                    span: 8,
                }}
                wrapperCol={{
                    span: 16,
                }}
                initialValues={{
                    remember: true,
                }}
                onFinish={onFinish}
                onFinishFailed={onFinishFailed}
                autoComplete="off"
            >
                <Form.Item
                    label="Username"
                    name="username"
                    rules={[
                        {
                            required: true,
                            message: 'Please input your username!',
                        },
                    ]}
                >
                    <Input />
                </Form.Item>

                <Form.Item
                    label="Password"
                    name="password"
                    rules={[
                        {
                            required: true,
                            message: 'Please input your password!',
                        },
                    ]}
                >
                    <Input.Password />
                </Form.Item>

                <Form.Item
                    name="remember"
                    valuePropName="checked"
                    wrapperCol={{
                        offset: 8,
                        span: 16,
                    }}
                >
                    <Checkbox>Remember me</Checkbox>
                </Form.Item>

                <Form.Item
                    wrapperCol={{
                        offset: 8,
                        span: 16,
                    }}
                >
                    <Button type="primary" htmlType="submit">
                        Submit
                    </Button>
                </Form.Item>
            </Form>
            {
                failedErrorInfo && <Alerts type="error" message={failedErrorInfo} />
            }
        </div>
    );
};

export default Login;