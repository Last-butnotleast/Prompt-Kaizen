import createClient from "openapi-fetch";
import type { paths } from "./generated/schema";
import supabase from "@/lib/supabase";

export const apiClient = createClient<paths>({
  baseUrl: import.meta.env.VITE_API_URL || "http://localhost:3000",
});

apiClient.use({
  async onRequest({ request }) {
    const {
      data: { session },
    } = await supabase.auth.getSession();
    if (session?.access_token) {
      request.headers.set("Authorization", `Bearer ${session.access_token}`);
    }
    return request;
  },
});
