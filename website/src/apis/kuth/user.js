import Request from './request';

export const GetUser = (id) => {
    return Request("/v1/users/" + id);
}

export const GetUsers = (limit, offset) => {
    return Request("/v1/users?limit=" + limit + "&offset=" + offset);
}

export const DeleteUser = (id) => {
    return Request("/v1/users/" + id, 'DELETE', 204);
}

export const CreateUser = (name, password, desc) => {
    return Request("/v1/users", "POST", 201, {
        name,
        password,
        desc,
    });
}