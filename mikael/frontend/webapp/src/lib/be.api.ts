import { useLoggedInUser } from "@/store/user";
import { api } from "./api";

export async function login(email: string, password: string) {
  const [user, err] = await api
    .request("/auth/login")
    .post({ email, password });

  if (err) {
    console.error(err);
    return;
  }
  useLoggedInUser.setState({ user });
}

export async function getAllUsers(pageIndex: number, pageSize: number) {
  return await api
    .request("/users/{id}/{otherId}", { id: "", otherId: "" })
    .delete();
}
