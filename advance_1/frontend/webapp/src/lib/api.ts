import axios from "axios";
import { ApiResponse } from "./types/user";

export async function apiRequest<TInput, TOutput>(
    apiPath: string, 
    input: TInput, 
    method: "post" | "get" = "post"): Promise<TOutput | null> {
  const api = axios.create({
    baseURL: "http://localhost:3000/api/",
    timeout: 10000,
    headers: { "X-Custom-Header": "foobar" },
  });


  const resp = method === "post" 
    ? await api.post<TInput, ApiResponse<TOutput>>(apiPath,    input  ) 
    : method === "get" 
      ? await api.post<TInput, ApiResponse<TOutput>>(apiPath,    input  ) 
      : null;

  if (!resp || resp.status !== 200 || resp.data.message !== "Success") {
    return null;
  } else {
    return resp.data.value;
  }
}