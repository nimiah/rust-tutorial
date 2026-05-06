"use client";

import { InfiniteData, useInfiniteQuery } from "@tanstack/react-query";
import { FeedList } from "./feed-list";
import { PostSkeleton } from "./post-skeleton";
import { FeedPage, Post } from "@/lib/type";
import { fetchFeed } from "@/lib/api";

interface FeedClientProps {
  initialPosts: Post[],
  initialCursor: string | null
}

export function FeedClient({
  initialPosts,
  initialCursor,
} : FeedClientProps) {
  // 1) Build initialData with correct shape: InfiniteData<FeedPage>
  const initialData: InfiniteData<FeedPage> = {
    pages: [{ items: initialPosts, nextCursor: initialCursor }],
    pageParams: [null],
  }

  // 2) useInfiniteQuery with generics and correct initialData
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    status,
    error,
  } = useInfiniteQuery<FeedPage, Error>({
    queryKey: ["feed"],
    queryFn: ({ pageParam }) => fetchFeed(pageParam as string | null, 10),
    initialData, // pass the correctly shaped initialData variable
    initialPageParam: null,
    getNextPageParam: (lastPage) => lastPage.nextCursor ?? null,
  })

  // 3) Flatten posts from pages
  const posts: Post[] = data?.pages.flatMap((p) => p.items) ?? []

  return (
    <div className="w-full">
      <FeedList
        posts={posts}
        onLoadMore={() => fetchNextPage()}
        hasMore={!!hasNextPage}
      />

      {isFetchingNextPage && (
        <div className="space-y-4 mt-4">
          <PostSkeleton />
          <PostSkeleton />
        </div>
      )}

      {status === "error" && (
        <div className="text-red-500 mt-4">Error: {String(error?.message)}</div>
      )}
    </div>
  )
}
