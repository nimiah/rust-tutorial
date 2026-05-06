"use client";

import { Header } from "./header";
import { LeftSidebar } from "./sidebar-left";
import { RightSidebar } from "./sidebar-right";

export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <Header />

      <div className="flex w-full max-w-7xl mx-auto pt-16">
        <aside className="hidden lg:block w-64 shrink-0"><LeftSidebar /></aside>

        <main className="flex flex-1 px-4 md:px-6">{children}</main>

        <aside className="hidden xl:block w-72 shrink-0"><RightSidebar /></aside>
      </div>
    </div>)
}
