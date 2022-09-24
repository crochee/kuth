import { BrowserRouter, Route, Routes } from "react-router-dom";
import Home from "../views/home";
import Login from "../views/login";
import NotFound from "../views/notfound";

const Router = () => {
    return <BrowserRouter>
        <Routes>
            <Route exact path="/login" element={<Login />} />
            <Route path="/" element={<Home />}>
                <Route path="expenses" element={<div>expenses</div>} />
                <Route path="invoices" element={<div>invoices</div>} />
                <Route path="iam" element={<div>iam</div>} />
                <Route path="about" element={<div>about</div>} />
            </Route>
            <Route path="*" element={<NotFound />} />
        </Routes>
    </BrowserRouter>
}

export default Router;