import type { AxiosResponse } from "axios";
import type { LoggedInUser, LoginRequest, User } from "./user";

export type ApiEndpoint =
  | {
      path: "/users/";
      method: "get";
      // input: { pageIndex: number; pageSize: number };
      output: User[];
    }
  | {
      path: "/users/{id}";
      pathParam: { id: string };
      method: "put";
      input: Partial<LoggedInUser>;
    }
  | {
      path: "/users/{id}/{otherId}";
      pathParam: { id: string; otherId: string };
      method: "delete";
    }
  | {
      path: "/auth/login";
      method: "post";
      input: LoginRequest;
      output: LoggedInUser;
    };

export type RequestOptions = {
  timeout?: number;
  signal?: AbortSignal;
};

export type PathParamWithOptions<T extends ApiEndpoint> = T extends { pathParam: infer I }
  ? [I & RequestOptions]
  : [options?: RequestOptions];
export type Input<T extends ApiEndpoint> = T extends { input: infer I }
  ? [input: I]
  : [];
export type Output<T extends ApiEndpoint> = T extends { output: infer I }
  ? I
  : never;

export type ApiResponse<T> = AxiosResponse<{
  message: string;
  value: T;
}>;
export type Response<T> = [T | null, { message: string; code: string } | null];

export type Api<TPath extends ApiEndpoint["path"]> = Extract<
  ApiEndpoint,
  { path: TPath }
>;

export type Request<
  TPath extends ApiEndpoint["path"],
  TEndpoint extends Api<TPath>,
> = {
  [TMethod in TEndpoint["method"]]: TMethod extends Api<TPath>["method"]
    ? (
        ...args: Input<Extract<ApiEndpoint, { path: TPath; method: TMethod }>>
      ) => Promise<
        Response<Output<Extract<ApiEndpoint, { path: TPath; method: TMethod }>>>
      >
    : never;
};
