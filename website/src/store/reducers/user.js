import { createSlice } from '@reduxjs/toolkit';

const User = createSlice({
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
    },
    reducers: {
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
                desc: "",
                email: null,
                check: 0,
                sex: null,
                image: null,
                token: "",
            };
        },
    }
})

export default User;