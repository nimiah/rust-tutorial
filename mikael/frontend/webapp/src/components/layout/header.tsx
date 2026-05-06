"use client";

import { Input } from "@/components/ui/input";
import { ModeToggle } from "../mode-toggle";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";

export function Header() {
  return (
    <header className="fixed top-0 left-0 right-0 h-16 border-b bg-background/80 backdrop-blur z-50">
      <div className="h-full max-w-7xl mx-auto flex items-center justify-between px-4">
        {/* Logo */}
        <div className="font-bold text-xl">
          Feed Clone
        </div>

        {/* Search */}
        <div className="hidden md:block w-80">
          <Input placeholder="Search..." />
        </div>

        {/* Action */}
        <div className="flex items-center gap-4">
          <ModeToggle />

          <Avatar className="h-8 w-8">
            <AvatarImage src="/avatar.jpg" />

            <AvatarFallback>MK</AvatarFallback>
          </Avatar>
        </div>
      </div>
    </header>
  )
}
