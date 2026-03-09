import { createBrowserRouter } from "react-router";
import App from "./App";
import { Users } from "./pages/Users";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <App />
  },
  {
    path: "/users",
    element: <Users />
  }
]) 
