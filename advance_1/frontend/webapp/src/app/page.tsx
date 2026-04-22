"use client";
import { CardSmall } from "@/components/CardSmall";
import Counting from "@/components/Counting";
import CountingProvider from "@/components/CountingProvider";
import Link from "next/link";

export default function Home() {
  return (
    <div className="flex flex-col gap-6">
      <CardSmall />

      <Counting></Counting>

      <Link href={"/demo"}>Go to demo</Link>
    </div>
  );
}
