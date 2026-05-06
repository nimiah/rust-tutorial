"use client";

import { Home, Settings, User } from "lucide-react";
import Link from "next/link";

const items = [
  { label: "Home", icon: Home, href: "/" },
  { label: "Profile", icon: User, href: "/profile" },
  { label: "Settings", icon: Settings, href: "/settings" },
];

export function LeftSidebar() {
  return (<nav className="p-4 space-y-2">
    {items.map(item => {
      const Icon = item.icon;

      return (
        <Link
          key={item.href}
          href={item.href}
          className="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent transition"
        >
          <Icon className="h-5 w-5" />
          <span>{ item.label }</span>
        </Link>
      )
    })}
  </nav>)
}
