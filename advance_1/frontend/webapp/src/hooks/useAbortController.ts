import { useEffect, useRef } from "react";

export default function useAbortController() {
  const abort = useRef(new AbortController());

  useEffect(() => {
    return () => {
      abort.current.abort();
    }
  }, [])

  return abort.current.signal;
}