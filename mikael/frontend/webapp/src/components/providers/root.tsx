"use client";

import { ReactNode } from "react";
import { ThemeProvider } from "./theme";
import { QueryProvider } from "./query";

interface RootProvidersProps {
  children: ReactNode;
}

/**
 * Root Providers - Composer pattern
 * Combines all isolated providers
 * Order matters: Theme → Query → App
 *
 * This approach allows:
 * - Easy to add/remove providers
 * - Easy to test each provider independently
 * - Clear dependency graph
 * - Follows composition pattern (not monolith)
 */
export function RootProviders({ children }: RootProvidersProps) {
  return (
    <ThemeProvider>
      <QueryProvider>
        {children}
      </QueryProvider>
    </ThemeProvider>
  );
}
