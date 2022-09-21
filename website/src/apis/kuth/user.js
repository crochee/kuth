import Request from './request';

export const GetUserInfo = (id) => {
    return Request("/v1/users/" + id, 'GET');
}