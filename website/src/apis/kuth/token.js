import Request, { CustomRequest } from "./config";

const PostTokens = (username, password) => {
    var requestOptions = {
        method: 'POST',
        headers: new Headers({
            "Authorization": "Basic " + btoa(username + ":" + password)
        }),
        redirect: 'follow'
    };
    return new Promise((resolve, reject) => {
        CustomRequest("/v1/tokens", requestOptions)
    }).then((response) = {

    }).catch((error) => {

    })
};