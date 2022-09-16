import { GetUserInfo } from "../../apis/kuth/user";
import { VerifyToken } from "../../apis/kuth/auth";
import { UserClear, UserSetInfo } from "../../store"

export const getUserInfo = () => (dispatch) => {
    return new Promise((resolve) => {
        VerifyToken().then((resp) => {
            if (resp.decision === "Allow") {
                GetUserInfo(resp.user_id).then((userInfo) => {
                    dispatch(UserSetInfo(userInfo));
                    resolve(userInfo);
                })
                return
            }
            dispatch(UserClear());
        })
    });
};