"use client";
import { useEffect, useState } from "react";

export default function Counting() {
  const [counter, setCounter] = useState<number>();
  const [stop, setStop] = useState(false);

  console.log("🚀 ~ Counting ~ stop:", { stop, counter });

  useEffect(() => {
    console.log("🚀 ~ Counting ~ counter:", counter);
    setCounter(10);

    return () => {
      console.log("🚀 ~ Counting ~ is diposed");
    };
  }, []);

  useEffect(() => {
    if (stop) return;

    setTimeout(() => {
      setCounter((counter) => (counter ? ++counter : counter));
    }, 1000);
  }, [counter, stop]);

  return (
    <div className="flex gap-2">
      {counter}{" "}
      <button
        className="cursor-pointer"
        onClick={() => setStop((stop) => !stop)}
      >
        {stop ? "Start" : "Stop"}
      </button>
    </div>
  );
}
