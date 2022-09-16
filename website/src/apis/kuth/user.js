import Request from './config';

export const GetUserInfo = (id) => {
    return Request("/v1/users/" + id)
}