import { useDispatch } from 'react-redux';
import { UserClear } from '../../store';
import { useNavigate } from "react-router-dom";
import {removeToken} from '../../utils/auth';

const Logout = () => {
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const onClick = () => {
        dispatch(UserClear())
        removeToken()
        navigate("/login", { replace: true });
    }
    return (
        <div onClick={onClick}>退出</div>
    )
}

export default Logout;