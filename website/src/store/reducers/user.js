import { createSlice } from '@reduxjs/toolkit';
import { getToken, } from '../../utils/auth';

export default createSlice({
    name: 'user',
    initialState: {
        id: "",
        account_id: "",
        admin: 0,
        name: "",
        desc: "test",
        email: null,
        check: 0,
        sex: null,
        image: null,
        token: getToken(),
    },
    reducers: {
        UserSetToken: (state, { payload }) => {
            state.token = payload.token
        },
        UserSetInfo: (state, { payload }) => {
            state.id = payload.id;
            state.account_id = payload.account_id;
            state.admin = payload.admin;
            state.name = payload.name;
            state.desc = payload.desc;
            state.email = payload.email;
            state.check = payload.check;
            state.sex = payload.sex;
            state.image = payload.image;
        },
        UserClear: (state) => {
            state = {
                id: "",
                account_id: "",
                admin: 0,
                name: "",
                desc: "test",
                email: null,
                check: 0,
                sex: null,
                image: null,
                token: "",
            }
        },
        UserGet: (state) => {
            return state
        }
    }
})

export const UserSetToken = "USER_SET_USER_TOKEN";
export const UserSetInfo = "USER_SET_USER_INFO";
export const UserClear = "USER_RESET_USER";