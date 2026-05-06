"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactNode, useMemo } from "react";

interface QueryProviderProps {
  children: ReactNode;
}

/**
 * Query Provider - Isolated responsibility
 * Only handles React Query state management
 * No dependencies on other providers
 */
export function QueryProvider({ children }: QueryProviderProps) {
  // Create client once per component instance (not per render)
  const queryClient = useMemo(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            staleTime: 1000 * 60 * 5, // 5 minutes
            gcTime: 1000 * 60 * 10, // 10 minutes
            retry: 1,
            refetchOnWindowFocus: false,
          },
        },
      }),
    []
  );

  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
