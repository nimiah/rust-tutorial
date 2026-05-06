import { useRef } from "react";
import { PostCard } from "./post-card";
import { useVirtualizer } from "@tanstack/react-virtual";
import { Post } from "@/lib/type";

interface FeedListProps {
  posts: Post[],
  onLoadMore: () => void,
  hasMore: boolean,
}

export function FeedList({
  posts,
  onLoadMore,
  hasMore,
} : FeedListProps) {
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: posts.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 130,
    overscan: 5,
    measureElement: (el) => el.getBoundingClientRect().height,
    onChange: (instance) => {
      const lastItem = instance.getVirtualItems().at(-1);
      if (lastItem && lastItem.index >= posts.length - 1 && hasMore) {
        console.log("-> feed-list hasMore: ", hasMore,
          " - posts.length: ", posts.length,
          // " - posts: ", posts,
          " - virtualItems.length: ", virtualItems.length, 
          " - virtualItems.lastIndex: ", virtualItems[virtualItems.length - 1]?.index,
          "\n --- virtualItems: ", virtualItems, 
        );
        console.log("-> feed-list lastItem: ", lastItem);
        onLoadMore();
      }
    },
  });

  const virtualItems = virtualizer.getVirtualItems();

  return (
      <div ref={parentRef} className="h-[calc(100vh-10rem)] overflow-auto p-2">
        <div
          style={{
            height: `${virtualizer.getTotalSize()}px`,
            position: "relative",
          }}
        >
          {virtualizer.getVirtualItems().map(virtualRow => {
          // {[...Array(10).keys()].map(index => {
            const post = posts[virtualRow.index];
            {/* const post = posts[index]; */}
            if (!post) return null;

            return (
              <div
                key={post.id}
                data-index={virtualRow.index}
                ref={virtualizer.measureElement}
                style={{
                  position: "absolute",
                  top: 0,
                  left: 0,
                  width: "100%",
                  transform: `translateY(${virtualRow.start}px)`
                }}
              >
                <PostCard post={post} />
              </div>
            )
          })}
        </div>
      </div>
  )
}
