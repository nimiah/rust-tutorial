import { LoggedInUser } from "@/lib/types/user";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

type UserStore = {
  user?: LoggedInUser | null;
  actions: {
    setUser(user: LoggedInUser | null): void;
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
