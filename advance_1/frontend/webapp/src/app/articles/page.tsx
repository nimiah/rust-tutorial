"use client";

import {
  Bell,
  Bookmark,
  Eye,
  Heart,
  Home,
  LogIn,
  RefreshCw,
  Search,
  Send,
  ShieldCheck,
  UserPlus,
  Users,
} from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";

type ApiResponse<T> = {
  message: string;
  value: T | null;
};

type Article = {
  id: number;
  owner_id: number;
  time_created: string;
  visibility: string;
  title: string;
  body: string | null;
  description: string | null;
  views: number;
  likes: number;
};

const apiBaseUrl = "http://localhost:3001";
const contacts = ["Tiểu Hoa", "Alice", "Bob", "Linh Nguyễn", "Minh Trần"];

function getInitials(name: string) {
  return name
    .trim()
    .split(/\s+/)
    .slice(0, 2)
    .map((part) => part.charAt(0).toUpperCase())
    .join("");
}

function formatDate(value: string) {
  return new Date(value).toLocaleString("vi-VN", {
    hour: "2-digit",
    minute: "2-digit",
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  });
}

async function requestApi<T>(
  path: string,
  token?: string,
  options: RequestInit = {},
): Promise<ApiResponse<T>> {
  const response = await fetch(`${apiBaseUrl}${path}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(options.headers || {}),
    },
  });

  const data = (await response.json()) as ApiResponse<T>;

  if (!response.ok) {
    throw new Error(data.message || "Request failed");
  }

  return data;
}

export default function ArticlesPage() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [token, setToken] = useState("");
  const [userName, setUserName] = useState("Tiểu Hoa");
  const [email, setEmail] = useState("tieuhoa@example.com");
  const [password, setPassword] = useState("123456");
  const [search, setSearch] = useState("");
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [newDescription, setNewDescription] = useState("");
  const [newBody, setNewBody] = useState("");

  const filteredArticles = useMemo(() => {
    const keyword = search.trim().toLowerCase();
    if (!keyword) return articles;

    return articles.filter((article) =>
      [article.title, article.description || "", article.body || ""]
        .join(" ")
        .toLowerCase()
        .includes(keyword),
    );
  }, [articles, search]);

  const loadArticles = useCallback(async () => {
    setLoading(true);
    setMessage("");

    try {
      const data = await requestApi<Article[]>("/api/articles");
      setArticles(data.value || []);
      setMessage("Đã tải dữ liệu article từ backend.");
    } catch (error) {
      setMessage(
        error instanceof Error ? error.message : "Không tải được article.",
      );
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    const savedToken = window.localStorage.getItem("mini-face-token");
    if (savedToken) {
      setToken(savedToken);
    }

    loadArticles();
  }, [loadArticles]);

  const handleRegister = async () => {
    setLoading(true);
    setMessage("");

    try {
      const data = await requestApi<number>("/api/user", undefined, {
        method: "POST",
        body: JSON.stringify({ name: userName, email, password }),
      });

      setMessage(
        `Đã tạo tài khoản ID ${data.value}. Bây giờ có thể đăng nhập.`,
      );
    } catch (error) {
      setMessage(
        error instanceof Error ? error.message : "Không tạo được tài khoản.",
      );
    } finally {
      setLoading(false);
    }
  };

  const handleLogin = async () => {
    setLoading(true);
    setMessage("");

    try {
      const data = await requestApi<string>("/api/auth/login", undefined, {
        method: "POST",
        body: JSON.stringify({ email, password }),
      });

      const authToken = data.value || "";
      setToken(authToken);
      window.localStorage.setItem("mini-face-token", authToken);
      setMessage("Đăng nhập thành công. Bây giờ bạn có thể đăng bài thật.");
    } catch (error) {
      setMessage(
        error instanceof Error ? error.message : "Không đăng nhập được.",
      );
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = () => {
    setToken("");
    window.localStorage.removeItem("mini-face-token");
    setMessage("Đã đăng xuất.");
  };

  const handleCreateArticle = async () => {
    if (!token) {
      setMessage("Bạn cần login trước khi đăng bài.");
      return;
    }

    if (!newTitle.trim()) {
      setMessage("Tiêu đề bài viết không được để trống.");
      return;
    }

    setLoading(true);
    setMessage("");

    try {
      const data = await requestApi<Article>("/api/articles", token, {
        method: "POST",
        body: JSON.stringify({
          title: newTitle,
          description: newDescription || null,
          body: newBody || null,
          visibility: "public",
        }),
      });

      if (data.value) {
        setArticles((current) => [data.value as Article, ...current]);
      }
      setNewTitle("");
      setNewDescription("");
      setNewBody("");
      setMessage("Đã đăng bài thật vào database.");
    } catch (error) {
      setMessage(
        error instanceof Error ? error.message : "Không đăng được bài.",
      );
    } finally {
      setLoading(false);
    }
  };

  const handleLikeArticle = async (articleId: number) => {
    try {
      const data = await requestApi<Article>(
        `/api/articles/${articleId}/like`,
        undefined,
        {
          method: "PATCH",
        },
      );

      if (data.value) {
        setArticles((current) =>
          current.map((article) =>
            article.id === articleId ? (data.value as Article) : article,
          ),
        );
      }
    } catch (error) {
      setMessage(
        error instanceof Error ? error.message : "Không like được bài.",
      );
    }
  };

  return (
    <div className="fixed inset-0 overflow-auto bg-[#f0f2f5] text-slate-950">
      <header className="sticky top-0 z-20 border-b border-slate-200 bg-white shadow-sm">
        <div className="grid h-14 grid-cols-[1fr_auto] items-center gap-3 px-4 lg:grid-cols-[320px_1fr_320px]">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-600 text-xl font-black text-white">
              m
            </div>
            <label className="hidden h-10 min-w-0 flex-1 items-center gap-2 rounded-full bg-slate-100 px-4 text-sm text-slate-500 sm:flex">
              <Search size={18} />
              <input
                value={search}
                onChange={(event) => setSearch(event.target.value)}
                placeholder="Tìm bài viết từ backend"
                className="min-w-0 flex-1 bg-transparent outline-none"
              />
            </label>
          </div>

          <nav className="hidden justify-center gap-2 md:flex">
            <TopNav active icon={<Home size={22} />} label="Bảng tin" />
            <TopNav icon={<Users size={22} />} label="Bạn bè" />
            <TopNav icon={<Bookmark size={22} />} label="Đã lưu" />
          </nav>

          <div className="flex items-center justify-end gap-2">
            <button
              type="button"
              className="flex h-10 w-10 items-center justify-center rounded-full bg-slate-100"
              aria-label="Thông báo"
            >
              <Bell size={19} />
            </button>
            <div className="flex h-10 w-10 items-center justify-center rounded-full bg-blue-100 text-sm font-bold text-blue-700">
              {getInitials(userName || "User")}
            </div>
          </div>
        </div>
      </header>

      <div className="mx-auto grid max-w-[1480px] gap-5 px-4 py-4 lg:grid-cols-[300px_minmax(0,720px)_300px]">
        <aside className="space-y-4">
          <section className="rounded-xl bg-white p-4 shadow-sm">
            <div className="mb-4 flex items-center gap-3">
              <div className="flex h-11 w-11 items-center justify-center rounded-full bg-blue-100 font-bold text-blue-700">
                {getInitials(userName || "User")}
              </div>
              <div>
                <h2 className="font-bold">{userName || "Người dùng demo"}</h2>
                <p className="text-xs text-slate-500">
                  {token ? "Đã đăng nhập" : "Chưa đăng nhập"}
                </p>
              </div>
            </div>

            <div className="space-y-3">
              <input
                value={userName}
                onChange={(event) => setUserName(event.target.value)}
                placeholder="Tên hiển thị"
                className="h-10 w-full rounded-lg border border-slate-200 px-3 text-sm outline-none focus:border-blue-500"
              />
              <input
                value={email}
                onChange={(event) => setEmail(event.target.value)}
                placeholder="Email"
                className="h-10 w-full rounded-lg border border-slate-200 px-3 text-sm outline-none focus:border-blue-500"
              />
              <input
                value={password}
                onChange={(event) => setPassword(event.target.value)}
                placeholder="Mật khẩu"
                type="password"
                className="h-10 w-full rounded-lg border border-slate-200 px-3 text-sm outline-none focus:border-blue-500"
              />
            </div>

            <div className="mt-4 grid grid-cols-2 gap-2">
              <button
                type="button"
                onClick={handleRegister}
                disabled={loading}
                className="inline-flex h-10 items-center justify-center gap-2 rounded-lg bg-slate-100 text-sm font-bold hover:bg-slate-200 disabled:opacity-60"
              >
                <UserPlus size={16} />
                Đăng ký
              </button>
              <button
                type="button"
                onClick={handleLogin}
                disabled={loading}
                className="inline-flex h-10 items-center justify-center gap-2 rounded-lg bg-blue-600 text-sm font-bold text-white hover:bg-blue-700 disabled:opacity-60"
              >
                <LogIn size={16} />
                Login
              </button>
            </div>

            {token && (
              <button
                type="button"
                onClick={handleLogout}
                className="mt-2 h-10 w-full rounded-lg border border-slate-200 text-sm font-bold hover:bg-slate-50"
              >
                Đăng xuất
              </button>
            )}
          </section>

          <section className="rounded-xl bg-white p-4 shadow-sm">
            <h2 className="font-bold">Trạng thái API</h2>
            <p className="mt-2 text-sm leading-6 text-slate-600">
              Backend: <span className="font-semibold">{apiBaseUrl}</span>
            </p>
            <p className="text-sm leading-6 text-slate-600">
              Token:{" "}
              <span className={token ? "text-emerald-600" : "text-rose-600"}>
                {token ? "đã có" : "chưa có"}
              </span>
            </p>
            {message && (
              <p className="mt-3 rounded-lg bg-slate-100 p-3 text-sm text-slate-700">
                {message}
              </p>
            )}
          </section>
        </aside>

        <main className="min-w-0 space-y-4">
          <section className="rounded-xl bg-white p-4 shadow-sm">
            <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <p className="text-sm font-semibold text-blue-600">
                  Dữ liệu thật từ backend
                </p>
                <h1 className="text-2xl font-black">Bảng tin article</h1>
                <p className="mt-1 text-sm text-slate-500">
                  Đang hiển thị {filteredArticles.length} / {articles.length}{" "}
                  bài viết.
                </p>
              </div>
              <button
                type="button"
                onClick={loadArticles}
                disabled={loading}
                className="inline-flex h-10 items-center justify-center gap-2 rounded-lg bg-blue-600 px-4 text-sm font-bold text-white hover:bg-blue-700 disabled:opacity-60"
              >
                <RefreshCw size={16} />
                Tải lại
              </button>
            </div>
          </section>

          <section className="rounded-xl bg-white p-4 shadow-sm">
            <div className="flex items-start gap-3">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-blue-100 text-sm font-bold text-blue-700">
                {getInitials(userName || "User")}
              </div>
              <div className="min-w-0 flex-1 space-y-3">
                <input
                  value={newTitle}
                  onChange={(event) => setNewTitle(event.target.value)}
                  placeholder="Tiêu đề bài viết"
                  className="h-10 w-full rounded-xl bg-slate-100 px-4 text-sm outline-none focus:ring-2 focus:ring-blue-200"
                />
                <input
                  value={newDescription}
                  onChange={(event) => setNewDescription(event.target.value)}
                  placeholder="Mô tả ngắn"
                  className="h-10 w-full rounded-xl bg-slate-100 px-4 text-sm outline-none focus:ring-2 focus:ring-blue-200"
                />
                <textarea
                  value={newBody}
                  onChange={(event) => setNewBody(event.target.value)}
                  placeholder="Nội dung bài viết"
                  rows={3}
                  className="w-full resize-none rounded-xl bg-slate-100 px-4 py-3 text-sm outline-none focus:ring-2 focus:ring-blue-200"
                />
                <button
                  type="button"
                  onClick={handleCreateArticle}
                  disabled={loading || !newTitle.trim()}
                  className="inline-flex h-10 w-full items-center justify-center gap-2 rounded-lg bg-blue-600 text-sm font-bold text-white hover:bg-blue-700 disabled:bg-slate-300"
                >
                  <Send size={16} />
                  Đăng bài thật
                </button>
              </div>
            </div>
          </section>

          <section className="space-y-4">
            {filteredArticles.map((article) => (
              <article
                key={article.id}
                className="rounded-xl bg-white p-4 shadow-sm"
              >
                <div className="flex items-start gap-3">
                  <div className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-slate-900 text-sm font-bold text-white">
                    U{article.owner_id}
                  </div>
                  <div className="min-w-0 flex-1">
                    <div className="flex items-start justify-between gap-3">
                      <div>
                        <h2 className="font-bold leading-tight">
                          {article.title}
                        </h2>
                        <p className="text-xs text-slate-500">
                          Owner #{article.owner_id} ·{" "}
                          {formatDate(article.time_created)}
                        </p>
                      </div>
                      <span className="rounded-full bg-emerald-50 px-3 py-1 text-xs font-bold text-emerald-700">
                        {article.visibility}
                      </span>
                    </div>

                    {article.description && (
                      <p className="mt-3 font-medium text-slate-700">
                        {article.description}
                      </p>
                    )}
                    <p className="mt-2 whitespace-pre-line leading-7 text-slate-700">
                      {article.body ||
                        "Bài viết này chưa có nội dung chi tiết."}
                    </p>
                  </div>
                </div>

                <div className="mt-4 grid grid-cols-2 border-t border-slate-100 pt-3 text-sm font-semibold text-slate-600">
                  <button
                    type="button"
                    onClick={() => handleLikeArticle(article.id)}
                    className="flex items-center justify-center gap-2 rounded-lg py-2 hover:bg-slate-100"
                  >
                    <Heart size={17} />
                    {article.likes} lượt thích
                  </button>
                  <div className="flex items-center justify-center gap-2 py-2">
                    <Eye size={17} />
                    {article.views} lượt xem
                  </div>
                </div>
              </article>
            ))}

            {!loading && filteredArticles.length === 0 && (
              <div className="rounded-xl bg-white p-8 text-center text-slate-500 shadow-sm">
                Chưa có article nào từ backend.
              </div>
            )}
          </section>
        </main>

        <aside className="hidden lg:block">
          <section className="rounded-xl bg-white p-4 shadow-sm">
            <h2 className="font-bold text-slate-700">Người liên hệ</h2>
            <div className="mt-3 space-y-1">
              {contacts.map((contact) => (
                <div
                  key={contact}
                  className="flex items-center gap-3 rounded-lg px-2 py-2 hover:bg-slate-100"
                >
                  <div className="relative">
                    <div className="flex h-9 w-9 items-center justify-center rounded-full bg-slate-100 text-xs font-bold">
                      {getInitials(contact)}
                    </div>
                    <span className="absolute bottom-0 right-0 h-3 w-3 rounded-full border-2 border-white bg-green-500" />
                  </div>
                  <p className="text-sm font-semibold">{contact}</p>
                </div>
              ))}
            </div>
          </section>

          <section className="mt-4 rounded-xl bg-white p-4 shadow-sm">
            <div className="flex items-center gap-2 text-blue-700">
              <ShieldCheck size={18} />
              <h2 className="font-bold">Backend thật</h2>
            </div>
            <p className="mt-3 text-sm leading-6 text-slate-600">
              Đọc bài dùng <code>/api/articles</code>, đăng bài dùng{" "}
              <code>POST /api/articles</code>, like dùng{" "}
              <code>PATCH /api/articles/:id/like</code>.
            </p>
          </section>
        </aside>
      </div>
    </div>
  );
}

function TopNav({
  active = false,
  icon,
  label,
}: {
  active?: boolean;
  icon: React.ReactNode;
  label: string;
}) {
  return (
    <button
      type="button"
      className={`flex h-12 w-28 items-center justify-center rounded-lg ${
        active
          ? "border-b-4 border-blue-600 text-blue-600"
          : "text-slate-500 hover:bg-slate-100"
      }`}
      aria-label={label}
    >
      {icon}
    </button>
  );
}
