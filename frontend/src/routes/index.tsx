import { PromptTestComponent } from "@/components/test";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  return (
    <main>
      <PromptTestComponent />
    </main>
  );
}
