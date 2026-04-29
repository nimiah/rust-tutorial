import axios, { type AxiosInstance } from "axios";
import type {
  Api,
  ApiEndpoint,
  ApiResponse,
  PathParam,
  Request,
  Response,
} from "./types/api";

const BACKEND_BASE_URL =
  process.env.NEXT_PUBLIC_BACKEND_BASE_URL || "http://localhost:4000/api/";

export const api = {
  request: <TPath extends ApiEndpoint["path"], TApi extends Api<TPath>>(
    path: TPath,
    ...params: PathParam<TApi>
  ): Request<TPath, TApi> => {
    const apiPath = params[0]
      ? Object.keys(params[0]).reduce<string>(
          (path, key) => path.replaceAll(`{${key}}`, (params[0] as any)[key]),
          path,
        )
      : path;

    return {
      get: (input: any) => {
        const params = new URLSearchParams(input);
        return request((axios) => axios.get(`${apiPath}?${params.toString()}`));
      },
      delete: () => request((axios) => axios.delete(apiPath)),
      put: (input: any) => request((axios) => axios.put(apiPath, input)),
      post: (input: any) => request((axios) => axios.post(apiPath, input)),
      patch: (input: any) => request((axios) => axios.patch(apiPath, input)),
    } as any;
  },
};

async function request<T>(
  fn: (axios: AxiosInstance) => Promise<ApiResponse<T>>,
): Promise<Response<T>> {
  const axiosInstance = axios.create({
    baseURL: BACKEND_BASE_URL,
    timeout: 10000,
    headers: { "Content-Type": "application/json" },
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
