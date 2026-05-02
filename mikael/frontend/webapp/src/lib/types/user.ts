import type { AxiosResponse } from "axios";

export type ApiResponse<T> = AxiosResponse<{
  message: string;
  value: T;
}>

export type LoginRequest = {
  email: string;
  password: string;
};

export type LoggedInUser = {
  email: string;
  name: string;
  token: string;
};