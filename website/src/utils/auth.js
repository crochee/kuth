const TokenKey = "token";

export const getToken = () => {
    return window.localStorage.getItem(TokenKey)
}

export const setToken = (token) => {
    return window.localStorage.setItem(TokenKey, token)
}

export const removeToken = () => {
    return window.localStorage.removeItem(TokenKey)
}