import { DashboardLayout } from "@/components/layout/DashboardLayout";

export function DashboardPage() {
  return (
    <DashboardLayout title="Dashboard">
      <div className="grid gap-4 md:grid-cols-3">
        <StatCard title="Prompts" value={0} />
        <StatCard title="Versions" value={0} />
        <StatCard title="Feedback" value={0} />
      </div>
      <div className="rounded-xl border bg-card p-6">
        <h2 className="text-lg font-semibold mb-2">Welcome to Prompt Kaizen</h2>
        <p className="text-muted-foreground">
          Start by creating your first prompt or explore the features from the
          sidebar.
        </p>
      </div>
    </DashboardLayout>
  );
}

function StatCard({ title, value }: { title: string; value: number }) {
  return (
    <div className="rounded-xl border bg-card p-6">
      <div className="text-3xl font-bold">{value}</div>
      <div className="text-sm text-muted-foreground">{title}</div>
    </div>
  );
}
