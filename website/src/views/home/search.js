import {
    AudioOutlined,
} from '@ant-design/icons';
import { Input, message } from 'antd';
import { useState } from 'react';
const suffix = (
    <AudioOutlined
        style={{
            fontSize: 16,
            color: '#1890ff',
        }}
    />
);

const Search = () => {
    let [loadingEnabled, loadingFc] = useState(false);
    const doSearch = (value) => {
        if (value) {
            loadingFc(true);
            console.log(value);
            loadingFc(false);
        } else {
            message.warning("输入不合法");
        }
    }
    return (loadingEnabled ? <Input.Search
        placeholder="input search text"
        allowClear
        onSearch={doSearch}
        enterButton
        suffix={suffix}
        loading
        style={{
            width: 200,
        }} /> : <Input.Search
        placeholder="input search text"
        allowClear
        onSearch={doSearch}
        enterButton
        suffix={suffix}
        style={{
            width: 200,
        }} />)
}

export default Search;