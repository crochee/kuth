import { message } from 'antd';
import { getToken } from '../../utils/auth';

export const KuthUrl = "http://127.0.0.1:30050";

export default (url, options) => {
    const defaultOptions = {
        /*容许携带cookies*/
        credentials: 'include',
        /*容许跨域**/
        mode: 'cors',
        headers: new Headers({
            Accept: 'application/json; charset=utf-8',
            Authorization: getToken(),
        }),
        body: null,
    }
    options.method = options.method || 'GET';
    options.code = options.code || 200;

    if (options.method == 'GET' | 'HEAD' | 'DELETE') {
        options.body = null;
    }
    if (options.body) {
        defaultOptions.headers.set('Content-Type', 'application/json; charset=utf-8');
        defaultOptions.body = JSON.stringify(options.body);
    }
    const f = new Promise((resolve, reject) => {
        fetch(KuthUrl + url, defaultOptions).then((response) => {
            if (response.status === 204) {
                return
            }
            if (response.status != options.code) {
                reject(response.json())
                return
            }
            let contentType = response.headers.get("Content-Type");
            if (contentType === "application/json") {
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