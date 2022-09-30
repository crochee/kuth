import { message } from 'antd';
import { getToken } from '../../utils/auth';

export const KuthUrl = "http://127.0.0.1:30050";

const Invoke = (url, method = 'GET', code = 200, data = null) => {
    const f = new Promise((resolve, reject) => {
        Request(url, method, code, data).then((response) => {
            if (response.status === 204) {
                return
            }
            if (response.status !== code) {
                reject(response.json())
                return
            }
            let contentType = response.headers.get("Content-Type");
            if (contentType.startsWith("application/json")) {
                resolve(response.json())
                return
            }
            throw new Error("Content-Type not supported" + contentType)
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

export default Invoke;

export const Request = (url, method = 'GET', code = 200, data = null) => {
    const options = {
        method: method,
        /*容许携带cookies*/
        credentials: 'include',
        /*容许跨域**/
        mode: 'cors',
        headers: {
            Accept: 'application/json; charset=utf-8',
            Authorization: "Bearer " + getToken(),
        },
        body: data,
    }
    if (options.method === 'GET' || options.method === 'HEAD' || options.method === 'DELETE') {
        options.body = null;
    }
    if (options.body) {
        options.headers = {
            ...options.headers,
            'Content-Type': 'application/json; charset=utf-8',
        };
        options.body = JSON.stringify(options.body);
    }
    return new Promise((resolve, reject) => {
        fetch(KuthUrl + url, options).then(resolve).catch(reject)
    })
}

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