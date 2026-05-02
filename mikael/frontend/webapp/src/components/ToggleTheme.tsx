"use client";

import { useTheme } from "next-themes";
import { Button } from "./ui/button";

export default function ToggleTheme() {
  const { theme, setTheme } = useTheme(); // react hook

  const toggleTheme = () => {
    setTheme((theme) => (theme === "light" ? "dark" : "light"));
  };

  return (
    <Button onClick={toggleTheme} className="fixed top-10 right-10">
      Switch
    </Button>
  );
}
