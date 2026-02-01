import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { Toaster } from "sonner";

const RootLayout = () => (
  <>
    <Outlet />
    <TanStackRouterDevtools />
    <Toaster richColors />
  </>
);

export const Route = createRootRoute({ component: RootLayout });
