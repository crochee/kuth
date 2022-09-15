const KuthUrl = "http://127.0.0.1:30050";

export const CustomRequest = (url, requestOptions) => {
    return new Promise((resolve, reject) => {
        fetch(KuthUrl + url, requestOptions).then(resolve).catch(reject)
    })
};

const Request = (url, requestOptions) => {
    var myHeaders = new Headers(requestOptions.headers);
    myHeaders.set('Accept', 'application/json')
    requestOptions.body && myHeaders.set('Content-Type', 'application/json');
    requestOptions.headers = myHeaders;

    return new Promise((resolve, reject) => {
        CustomRequest(url, requestOptions).then((response) => {
            if (response.status === 204) {
                return
            }
            let contentType = response.headers.get("Content-Type");
            // json
            if (contentType === "application/json") {
                resolve(response.json())
                return
            }
            throw new Error("Content-Type not supported" + contentType)
        }).catch(reject)
    })
};

export default Request;