import axios from "axios";
import type { ApiError } from "../entities/ApiError";

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || "http://localhost:5005/api",
  // baseURL:  "/api",
});

api.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (!error.response) {
      const networkErr: ApiError = {
        message: "Сетевая ошибка. Проверьте подключение.",
      };
      return Promise.reject(networkErr);
    }
    // Response present
    const resp = error.response;
    const data = resp.data || {};

    let apiError: ApiError = {};

    if (!(await data.text())) {
      apiError = {
        message: data.message || resp.statusText,

        details: data.details,
      };
    } else {
      const respJson = JSON.parse(await data.text());
      if ((respJson as ApiError)?.message) {
        apiError = {
          ...respJson,
        };
      } else {
        apiError = {
          message: data.message || resp.statusText,
          details: data.details,
        };
      }
    }
    // Доп. логика: 401 -> refresh token flow
    if (resp.status === 401) {
      // handle refresh or redirect to login
    }

    return Promise.reject(apiError);
  }
);
