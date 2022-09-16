import { Avatar, Image } from 'antd';

export default (props) => {
    const {
        userName,
        imageUrl,
    } = props
    return imageUrl ? <Avatar className="user" src={<Image src={imageUrl} style={{ width: 32 }} />} /> :
        <Avatar className="user" style={{ color: '#f56a00', backgroundColor: '#fde3cf' }}>{userName}</Avatar>
}