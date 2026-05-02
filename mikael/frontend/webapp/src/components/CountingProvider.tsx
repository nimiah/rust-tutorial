"use client";
import { createContext, useContext, useState } from "react";

type CountingControlsProps = {
  counter: number;
  increase(): void;
  decrease(): void;
  reset(): void;
};

const CountingContext = createContext(null as unknown as CountingControlsProps);

export default function CountingProvider({ children }: any) {
  const [counter, setCounter] = useState<number>(0);

  return (
    <CountingContext.Provider
      value={{
        counter,
        increase: () => setCounter((counter) => ++counter),
        decrease: () => setCounter((counter) => --counter),
        reset: () => setCounter(0),
      }}
    >
      {children}
    </CountingContext.Provider>
  );
}

export function useCounter() {
  return useContext(CountingContext);
}
