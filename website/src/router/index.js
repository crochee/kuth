import React from "react";
import { HashRouter, Route, Routes, Navigate, Outlet } from "react-router-dom";
import { connect } from "react-redux";
import Home from "../views/home";
import Login from "../views/login";
import { getUserInfo } from "../store/actions/user";

const Router = (props) => {
    return <HashRouter>
        <Routes>
            <Route element={<Outlet />}>
                <Route path="/login" element={<Login />} />
                <Route
                    path="/"
                    element={<SwitchRouter {...props} />}
                />
            </Route>
        </Routes>
    </HashRouter>
}

export default connect((state) => state.user, { getUserInfo })(Router);


const SwitchRouter = (props) => {
    const { token, id, getUserInfo } = props;
    console.log(token, id);
    if (!token) {
        console.log("go to login");
        return <Navigate to="/login" />;
    }
    if (id) {
        console.log("go to home");
        return <Home />;
    }
    console.log("go to getUserInfo");
    getUserInfo().then(() => <Home />);
}