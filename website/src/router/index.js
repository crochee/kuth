import React from "react";
import { HashRouter, Route, Routes, Navigate, useLocation, Outlet } from "react-router-dom";
import Home from "../views/home";
import Login from "../views/login";
import { useSelector, useDispatch } from 'react-redux';
import { GetUserInfo } from "../apis/kuth/user";
import { VerifyToken } from "../apis/kuth/auth";
import { UserSetInfo, UserClear } from "../store";
import { removeToken } from '../utils/auth';

export default () => {
    return <HashRouter>
        <Routes>
            <Route element={<Outlet />}>
                <Route path="/login" element={<Login />} />
                <Route
                    path="/"
                    element={<WrapRouter />}
                />
            </Route>
        </Routes>
    </HashRouter>
}


const WrapRouter = () => {
    const userStore = useSelector((state) => state.user);
    const dispatch = useDispatch();
    let location = useLocation();
    console.log(userStore);
    if (!userStore.token) {
        return <Navigate to="/login" state={{ from: location }} replace />;
    }
    if (userStore.id) {
        return <Home />;
    }

    VerifyToken().then(response => {
        if (response.decision === 'Allow') {
            GetUserInfo(response.user_id).then(resp => {
                dispatch(UserSetInfo(resp))
            })
            return
        }
        dispatch(UserClear())
        removeToken();
    })
}