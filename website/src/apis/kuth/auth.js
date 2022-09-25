import Request, { KuthUrl } from "./request";
import { message } from 'antd';
import { getToken } from '../../utils/auth';

export const PostTokens = (username, password) => {
    var options = {
        method: 'POST',
        /*容许携带cookies*/
        credentials: 'include',
        /*容许跨域**/
        mode: 'cors',
        headers: {
            Authorization: "Basic " + btoa(username + ":" + password)
        },
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
    var options = {
        method: 'POST',
        /*容许携带cookies*/
        credentials: 'include',
        /*容许跨域**/
        mode: 'cors',
        headers: {
            Accept: 'application/json; charset=utf-8',
            Authorization: "Bearer " + getToken(),
        },
    };
    const f = new Promise((resolve, reject) => {
        fetch(KuthUrl + "/v1/auth?action=post&path=/v1/auth", options).then((response) => {
            if (response.status === 200) {
                resolve(response.json())
                return
            }
            let contentType = response.headers.get("Content-Type");
            if (contentType === "application/json") {
                reject(response.json())
                return
            }
            throw new Error("Content-Type not supported" + contentType)
        }).catch((error) => {
            message.error(error)
        })
    })
    return new Promise((resolve, reject) => {
        f.then(resolve).catch(
            (result) => {
                result.then((content) => {
                    reject(result);
                    message.warn(content.message, 5)
                })
            }
        )
    })

    return Request("/v1/auth?action=post&path=/v1/auth", "POST")
}