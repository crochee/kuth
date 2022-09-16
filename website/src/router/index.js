import React from "react";
import { HashRouter, Route, Routes, Navigate } from "react-router-dom";
import { connect } from "react-redux";
import Home from "../views/home";
import Login from "../views/login";
import { getUserInfo } from "../store/actions/user";

const Router = (props) => {
    const { token, id, getUserInfo } = props;
    return <HashRouter>
        <Routes>
            <Route exact path="/login" component={Login} />
            <Route
                path="/"
                render={() => {
                    if (!token) {
                        return <Navigate to="/login" />;
                    }
                    if (id) {
                        return <Home />;
                    }
                    getUserInfo().then(() => <Home />);
                }}
            />
        </Routes>
    </HashRouter>
}

export default connect((state) => state.user, { getUserInfo })(Router);



