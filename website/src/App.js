import React from "react";
import { Provider } from "react-redux";
import { ConfigProvider } from "antd";
import zhCN from "antd/es/locale/zh_CN";
import Router from "./router";
import store from "./store";

export default () => {
  return (
    <ConfigProvider locale={zhCN}>
      <Provider store={store}>
        <Router />
      </Provider>
    </ConfigProvider>
  );
};