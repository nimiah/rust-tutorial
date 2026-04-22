import { create } from "zustand";
import { persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

type CountingStore = {
  counter: number;
  actions: {
    increase(): void;
    decrease(): void;
    reset(): void;
  };
};

export const useCounterStore = create<CountingStore>()(
  immer(
    persist(
      (set) => ({
        counter: 0,
        actions: {
          // increase: () => set((s) => ({ ...s, counter: s.counter + 1 })),
          // decrease: () => set((s) => ({ ...s, counter: s.counter - 1 })),
          // reset: () => set((s) => ({ ...s, counter: 0 })),
          increase: () =>
            set((state) => {
              console.log("🚀 ~ state:", state);
              state.counter = state.counter + 1;
            }),
          decrease: () =>
            set((state) => {
              state.counter = state.counter - 1;
            }),
          reset: () =>
            set((state) => {
              state.counter = 0;
            }),
        },
      }),
      {
        name: "counting",
        partialize: (state) => ({
          counter: state.counter, // 👈 persist ONLY state
        }),
      },
    ),
  ),
);
