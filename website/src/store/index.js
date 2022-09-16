import { applyMiddleware, combineReducers, configureStore } from '@reduxjs/toolkit';
import reduxThunk from 'redux-thunk';
import user from './reducers/user';

export const { UserSetToken, UserSetInfo, UserClear, UserGet } = user.actions;

export default configureStore(combineReducers({
    user: user.reducer,
}), applyMiddleware(reduxThunk));