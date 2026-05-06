import axios, { type AxiosInstance } from "axios";
import type {
  Api,
  ApiEndpoint,
  ApiResponse,
  PathParamWithOptions,
  Request,
  RequestOptions,
  Response,
} from "./types/api";
import { useLoggedInUser } from "@/store/user";

const BACKEND_BASE_URL =
  process.env.NEXT_PUBLIC_BACKEND_BASE_URL || "http://localhost:4000/api/";

export const api = {
  request: <TPath extends ApiEndpoint["path"], TApi extends Api<TPath>>(
    path: TPath,
    ...params: PathParamWithOptions<TApi>
  ): Request<TPath, TApi> => {
    const pathWithOptions = params[0];
    const apiPath = pathWithOptions
      ? Object.keys(params).reduce<string>(
          (acc, key) => acc.replace(`{${key}}`, (params as any)[key]),
          path,
        )
      : path;
    

    return {
      get: (input: any) => {
        const params = new URLSearchParams(input);
        return request(
          (axios) => axios.get(`${apiPath}?${params.toString()}`),
          pathWithOptions,
        );
      },
      delete: () => request((axios) => axios.delete(apiPath), pathWithOptions),
      put: (input: any) =>
        request((axios) => axios.put(apiPath, input), pathWithOptions),
      post: (input: any) =>
        request((axios) => axios.post(apiPath, input), pathWithOptions),
      patch: (input: any) =>
        request((axios) => axios.patch(apiPath, input), pathWithOptions),
    } as any;
  },
};

async function request<T>(
  fn: (axios: AxiosInstance) => Promise<ApiResponse<T>>,
  options?: RequestOptions
): Promise<Response<T>> {
  const token = useLoggedInUser.getState().user?.token;
  const axiosInstance = axios.create({
    baseURL: BACKEND_BASE_URL,
    timeout: options?.timeout || 10000,
    signal: options?.signal,
    headers: {
      "Content-Type": "application/json",
      ...(token && { Authorization: `Bearer ${token}` }),
    },
  });

  const resp = await fn(axiosInstance);
  if (resp.status === 200 && resp.data.message === "Success") {
    return [resp.data.value, null];
  }

  return [
    null,
    {
      code: "SERVER_ERROR",
      message: resp.data.message,
    },
  ];
}
