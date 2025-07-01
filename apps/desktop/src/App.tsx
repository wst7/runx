import { lazy } from "react";
import {
  createBrowserRouter,
  Outlet,
  RouterProvider,
} from "react-router";
import ThemeProvider from "./ThemeProvider";
import "./App.css";

const Root = () => {
  return (
    <>
      <Outlet />
    </>
  );
};

let router = createBrowserRouter([
  {
    Component: Root,
    children: [
      {
        index: true,
        Component: lazy(() => import("@/pages/RunCode")),
      },
      {
        path: "settings",
        Component: lazy(() => import("@/pages/Settings")),
      },
    ],
  },
]);
export default function App() {

  return (
    <ThemeProvider>
      <RouterProvider router={router} />
    </ThemeProvider>
  );
}
