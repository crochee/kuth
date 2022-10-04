import { FrownOutlined } from '@ant-design/icons';
import { Layout } from 'antd';

const NotFound = () => {
    return <Layout>
        <FrownOutlined />
        <Layout>404 Not Found</Layout>
        <Layout>Sorry, the page could not be found.</Layout>
    </Layout>
}

export default NotFound;