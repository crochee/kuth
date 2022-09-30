import { combineReducers, configureStore } from '@reduxjs/toolkit';
import thunk from 'redux-thunk';
import User from './reducers/user';

export const { UserSetInfo, UserClear } = User.actions;

const Store = configureStore({
    reducer: combineReducers({
        user: User.reducer,
    }),
    middleware: [thunk],
});

export default Store;