import { BrowserRouter, Route, Routes, Navigate } from "react-router-dom";
import Home from "../views/home";
import Login from "../views/login";
import NotFound from "../views/notfound";
import Iam from "../views/iam";
import UserList from "../views/iam/user";

const Router = () => {
    return <BrowserRouter>
        <Routes>
            <Route exact path="/login" element={<Login />} />
            <Route path="/" element={<Home />}>
                <Route path="expenses" element={<div>expenses</div>} />
                <Route path="invoices" element={<div>invoices</div>} />
                <Route path="iam" element={<Navigate to="/iam/users" />} />
                <Route path="iam" element={<Iam />}>
                    <Route path="users" element={<UserList />} />
                    <Route path="groups" element={<div>groups</div>} />
                    <Route path="secrets" element={<div>secrets</div>} />
                    <Route path="policys" element={<div>policys</div>} />
                </Route>
                <Route path="about" element={<div>about</div>} />
            </Route>
            <Route path="*" element={<NotFound />} />
        </Routes>
    </BrowserRouter>
}

export default Router;