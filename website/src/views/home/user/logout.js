import { useDispatch } from 'react-redux';
import { UserClear } from '../../../store';
import { useNavigate } from "react-router-dom";

const Logout = () => {
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const onClick = () => {
        dispatch(UserClear())
        navigate("/login", { replace: true });
    }
    return (
        <div onClick={onClick}>退出</div>
    )
}

export default Logout;