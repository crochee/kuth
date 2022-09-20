import Request, { KuthUrl } from "./config";
import { message } from 'antd';

export const PostTokens = (username, password) => {
    var options = {
        method: 'POST',
        headers: new Headers({
            "Authorization": "Basic " + btoa(username + ":" + password)
        }),
    };
    const f = new Promise((resolve, reject) => {
        fetch(KuthUrl + "/v1/auth/tokens", options).then((response) => {
            if (response.status === 200) {
                resolve({
                    Account: response.headers.get("X-Account-Id"),
                    User: response.headers.get("X-User-Id"),
                    Token: response.headers.get("X-Auth-Token")
                })
                return
            }
            reject(response.json())
        }).catch((error) => {
            message.error(error)
        })
    })
    return new Promise((resolve) => {
        f.then(resolve).catch(
            (result) => {
                result.then((content) => {
                    message.warn(content.message, 5)
                })
            }
        )
    })
};

export const VerifyToken = () => {
    return Request("/v1/auth?action=post&path=/v1/auth", { method: "POST" })
}