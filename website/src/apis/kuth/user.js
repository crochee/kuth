import Request from './request';

export const GetUserInfo = (id) => {
    return Request("/v1/users/" + id);
}

export const GetUserList = (limit, offset) => {
    return Request("/v1/users?limit=" + limit + "&offset=" + offset);
}