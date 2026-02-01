import { useNavigate } from "@tanstack/react-router";
import supabase from "@/lib/supabase";
import type { User } from "@supabase/supabase-js";

interface SimpleHeaderProps {
  user: User;
  breadcrumbs?: { label: string; to?: string }[];
  action?: React.ReactNode;
}

export function SimpleHeader({ user, breadcrumbs, action }: SimpleHeaderProps) {
  const navigate = useNavigate();

  const handleSignOut = async () => {
    await supabase.auth.signOut();
    navigate({ to: "/login" });
  };

  return (
    <header className="border-b bg-white">
      <div className="max-w-7xl mx-auto px-8 h-16 flex items-center justify-between">
        <div className="flex items-center gap-8">
          <button
            onClick={() => navigate({ to: "/prompts" })}
            className="flex items-center gap-2 hover:opacity-70"
          >
            <img src="/raccoon.svg" alt="Logo" width={24} height={24} />
            <span className="font-medium">Prompt Kaizen</span>
          </button>
          {breadcrumbs && breadcrumbs.length > 0 && (
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              {breadcrumbs.map((crumb, i) => (
                <div key={i} className="flex items-center gap-2">
                  {i > 0 && <span>/</span>}
                  {crumb.to ? (
                    <button
                      onClick={() => navigate({ to: crumb.to })}
                      className="hover:text-foreground"
                    >
                      {crumb.label}
                    </button>
                  ) : (
                    <span className="text-foreground">{crumb.label}</span>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
        <div className="flex items-center gap-6">
          {action}
          <div className="flex items-center gap-4 text-sm">
            <span className="text-muted-foreground">
              {user.user_metadata?.full_name ||
                user.user_metadata?.name ||
                user.email}
            </span>
            <button
              onClick={handleSignOut}
              className="text-muted-foreground hover:text-foreground"
            >
              Sign out
            </button>
          </div>
        </div>
      </div>
    </header>
  );
}
