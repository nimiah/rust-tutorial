import { FeedClient } from "@/app/feed/page";
import { fetchFeed } from "@/lib/api";
import { FeedPage, Post } from "@/lib/type";

const MOCK_POSTS = [
  {
    id: "1",
    author: "Alice",
    avatar: "https://images.stockcake.com/public/2/3/7/23754627-0a14-4855-971d-49de7b8aecfc_large/intense-anime-portrait-stockcake.jpg",
    content: "Hôm nay trời đẹp quá, đi dạo quanh hồ Tây 🌅",
    images: [
      "https://img.freepik.com/premium-photo/cute-anime-boy-wallpaper_776894-110627.jpg?semt=ais_hybrid&w=740&q=80",
      "https://png.pngtree.com/background/20250709/original/pngtree-anime-girl-cityscape-night-view-beautiful-picture-image_16695392.jpg"
    ],
    createdAt: "2026-04-19 07:00",
  },
  {
    id: "2",
    author: "Bob",
    avatar: "https://img.freepik.com/premium-photo/cute-anime-boy-wallpaper_776894-110627.jpg?semt=ais_hybrid&w=740&q=80",
    content: "Đang thử code Next.js + Axum, mọi thứ chạy mượt 😎",
    images: [],
    createdAt: "2026-04-19 06:45",
  },
  {
    id: "3",
    author: "Charlie",
    avatar: "https://png.pngtree.com/background/20250709/original/pngtree-anime-girl-cityscape-night-view-beautiful-picture-image_16695392.jpg",
    content: "Cafe sáng cùng team, chuẩn bị sprint mới ☕",
    images: [
      "https://m.media-amazon.com/images/I/6154Bwvu4vL._AC_UF894,1000_QL80_.jpg"
    ],
    createdAt: "2026-04-19 06:30",
  },
  {
    id: "4",
    author: "Diana",
    avatar: "https://m.media-amazon.com/images/I/6154Bwvu4vL._AC_UF894,1000_QL80_.jpg",
    content: "Đọc sách Rust Programming, thấy nhiều thứ hay ho 📚",
    images: [],
    createdAt: "2026-04-19 06:15",
  },
  {
    id: "5",
    author: "Eve",
    avatar: "https://www.shutterstock.com/image-vector/anime-boy-wearing-mask-black-600nw-2563625869.jpg",
    content: "Chạy bộ sáng nay 5km quanh công viên 🌳",
    images: [
      "https://img.freepik.com/premium-photo/cute-anime-boy-wallpaper_776894-110627.jpg?semt=ais_hybrid&w=740&q=80"
    ],
    createdAt: "2026-04-19 06:00",
  },
];

async function getInitialFeed(limit = 10): Promise<FeedPage> {
  const data = fetchFeed(null, limit);
  return data;
}

export default async function Home() {
  const initialPage = await getInitialFeed(10);
  const initialPosts: Post[] = initialPage.items;
  const initialCursor = initialPage.nextCursor;

  return (
    <div className="flex flex-col flex-1 w-full justify-center font-sans bg-white dark:bg-black">
      {/* <FeedClient initialPosts={MOCK_POSTS} initialCursor={"6"} /> */}
      <FeedClient initialPosts={initialPosts} initialCursor={initialCursor} />
    </div>
  );
}
