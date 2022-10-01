import { BrowserRouter, Route, Routes, Navigate } from "react-router-dom";
import Home from "../views/home";
import Login from "../views/login";
import NotFound from "../views/notfound";
import Iam from "../views/iam";
import Users, { User } from "../views/iam/user";
import Groups, { Group } from "../views/iam/group";

const Router = () => {
    return <BrowserRouter>
        <Routes>
            <Route exact path="/login" element={<Login />} />
            <Route path="/" element={<Navigate to="/home" />} />
            <Route path="/" element={<Home />}>
                <Route path="home" element={<div>Welcome</div>} />
                <Route path="expenses" element={<div>expenses</div>} />
                <Route path="invoices" element={<div>invoices</div>} />
                <Route path="iam" element={<Navigate to="/iam/users" />} />
                <Route path="iam" element={<Iam />}>
                    <Route path="users" element={<Users />} />
                    <Route path="users/:id" element={<User />} />
                    <Route path="groups" element={<Groups />} />
                    <Route path="groups/:id" element={<Group />} />
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