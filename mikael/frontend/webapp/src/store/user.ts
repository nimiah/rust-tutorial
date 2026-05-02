/** biome-ignore-all lint/suspicious/noAssignInExpressions: <explanation> */
import { apiRequest } from "@/lib/api";
import { LoggedInUser, LoginRequest } from "@/lib/types/user";
import { login } from "@/lib/be.api";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

type UserStore = {
  user?: LoggedInUser | null;
  actions: {
    setUser(user: LoggedInUser | null): void;
    // login(email: string, password: string): void;
  };
};

export const useLoggedInUser = create<UserStore>()(
  immer(
    persist(
      (set) => ({
        actions: {
          setUser: (user) =>
            set((state) => {
              state.user = user;
            }),
          // login: (email: string, password: string) =>
          //   set(async (state) => {
          //     const user = await apiRequest<LoginRequest, LoggedInUser>(
          //       "/auth/login",
          //       {
          //         email,
          //         password,
          //       },
          //     );
          //     state.user = user;
          //   }),
        },
      }),
      {
        name: "userLoggedIn",
        partialize: (state) => ({
          user: state.user, // 👈 persist ONLY state
        }),
      },
    ),
  ),
);
