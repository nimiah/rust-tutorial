"use client";
import useAbortController from "@/hooks/useAbortController";
import { api } from "@/lib/api";
import { User } from "@/lib/types/user";
import Link from "next/link";
import { setuid } from "process";
import { useEffect, useState } from "react";

export default function Demo() {
  return (
    <>
      <Link href={"/"}>Home</Link>
      <UserList />
    </>
  );
}

function UserList() {
  const signal = useAbortController();
  const [users, setUsers] = useState<User[]>([]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
  useEffect(() => {
    void (async () => {
      const [users, error] = await api
        .request("/users", { signal, timeout: 20000 })
        .get();
      if (error === null && !!users) setUsers(users);
    })();

    // api.request("/users/").get().then(([users, error]) => {
    //   if (error === null && !!users) setUsers(users);
    // })
  }, []);

  return (
    <ul>
      {users.map((user) => (
        <li key={user.id}>
          Id: {user.id} - name: {user.name} - email: {user.email}
        </li>
      ))}
    </ul>
  );
}
