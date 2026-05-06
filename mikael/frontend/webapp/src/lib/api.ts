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

import { FeedPage, Post } from "./type"
import { faker } from "@faker-js/faker"

// MOCK implementation (simulate network)
export async function fetchFeed(cursor?: string | null, limit = 10): Promise<FeedPage> {
  console.log(" *** api.ts fetchFeed cursor: ", cursor);

  // Simulate network latency
  await new Promise((r) => setTimeout(r, 400));

  // If cursor is null/undefined -> start from 1
  const start = cursor ? Number(cursor) : 1
  const items: Post[] = Array.from({ length: limit }).map((_, i) => {
    const id = String(start + i)
    return {
      id,
      author: `${faker.person.fullName()} ${id}`,
      avatar: faker.image.avatar(),
      content: `Mock post #${id} — nội dung demo để test infinite scroll. ${faker.lorem.paragraph((start + i) % 5)}`,
      images: faker.helpers.multiple(
        () => faker.image.urlPicsumPhotos({ width: 512, height: 512 }),
        { count: { min: 0, max: 4 } },
      ),
      createdAt: faker.date.past().toISOString(),
    }
  });

  // nextCursor: null when no more pages (for demo, stop at 100)
  const nextCursor = start + limit > 100 ? null : String(start + limit);

  console.log(" *** api.ts fetchFeed items: ", items);
  return { items, nextCursor }
}
