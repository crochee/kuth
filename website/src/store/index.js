import { applyMiddleware, combineReducers, configureStore } from '@reduxjs/toolkit';
import thunk from 'redux-thunk';
import user from './reducers/user';

export const { UserSetToken, UserSetInfo, UserClear, UserGet } = user.actions;

export default configureStore({
    reducer: combineReducers({
        user: user.reducer,
    }),
    middleware: [thunk],
});