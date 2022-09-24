import React from "react";
import { Provider } from "react-redux";
import { ConfigProvider } from "antd";
import zhCN from "antd/es/locale/zh_CN";
import Router from "./router";
import Store from "./store";

const App = () => {
  return (
    <ConfigProvider locale={zhCN}>
      <Provider store={Store}>
        <Router />
      </Provider>
    </ConfigProvider>
  );
};

export default App;