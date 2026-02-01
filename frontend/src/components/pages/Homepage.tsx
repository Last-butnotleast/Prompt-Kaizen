import { Link } from "@tanstack/react-router";

export function Homepage() {
  return (
    <div className="min-h-screen bg-linear-to-b from-white to-gray-50">
      <nav className="container mx-auto px-6 py-6 flex justify-between items-center">
        <div className="flex items-center gap-3">
          <img src="/raccoon.svg" alt="Logo" width={40} height={40} />
          <span className="text-xl font-bold">Prompt Kaizen</span>
        </div>
        <Link
          to="/login"
          className="px-6 py-2 bg-gray-900 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
        >
          Sign In
        </Link>
      </nav>

      <main className="container mx-auto px-6">
        <section className="max-w-4xl mx-auto text-center py-20">
          <img
            src="/raccoon.svg"
            alt="Prompt Kaizen"
            width={128}
            height={128}
            className="mx-auto mb-8"
          />
          <h1 className="text-6xl font-bold mb-6 bg-linear-to-r from-gray-900 to-gray-600 bg-clip-text text-transparent">
            Continuous Improvement for AI Prompts
          </h1>
          <p className="text-xl text-gray-600 mb-10 max-w-2xl mx-auto">
            Version, test, and systematically improve your AI prompts based on
            real user feedback and AI-driven insights.
          </p>
          <Link
            to="/login"
            className="inline-block px-8 py-4 bg-gray-900 text-white text-lg rounded-lg font-semibold hover:bg-gray-800 transition-colors shadow-lg"
          >
            Get Started
          </Link>
        </section>

        <section className="max-w-6xl mx-auto py-20">
          <h2 className="text-3xl font-bold text-center mb-16">
            Stop Guessing. Start Improving.
          </h2>

          <div className="grid md:grid-cols-3 gap-8">
            <FeatureCard
              title="Prompt Version Control"
              description="Track every change with Docker-style versioning. Tag releases, maintain history, and roll back when needed."
              icon="📦"
            />
            <FeatureCard
              title="Feedback Collection"
              description="Capture user ratings and comments directly linked to prompt versions. See what works and what doesn't."
              icon="⭐"
            />
            <FeatureCard
              title="AI-Powered Improvements"
              description="Get data-driven suggestions based on aggregated feedback patterns and best practices."
              icon="🚀"
            />
          </div>
        </section>

        <section className="max-w-4xl mx-auto py-20 text-center">
          <h2 className="text-3xl font-bold mb-6">
            The Problem with Ad-Hoc Prompting
          </h2>
          <div className="bg-white rounded-xl p-8 shadow-sm border border-gray-200">
            <ul className="text-left space-y-4 text-gray-700">
              <li className="flex items-start gap-3">
                <span className="text-red-500 font-bold">✗</span>
                <span>
                  No visibility into what prompt versions are deployed
                </span>
              </li>
              <li className="flex items-start gap-3">
                <span className="text-red-500 font-bold">✗</span>
                <span>Changes based on hunches instead of data</span>
              </li>
              <li className="flex items-start gap-3">
                <span className="text-red-500 font-bold">✗</span>
                <span>Lost history when someone "improves" a prompt</span>
              </li>
              <li className="flex items-start gap-3">
                <span className="text-red-500 font-bold">✗</span>
                <span>No way to track which version caused issues</span>
              </li>
            </ul>
          </div>
        </section>

        <section className="max-w-4xl mx-auto py-20 text-center">
          <h2 className="text-3xl font-bold mb-12">Integrate Anywhere</h2>
          <div className="grid md:grid-cols-3 gap-6">
            <IntegrationCard
              title="REST API"
              description="Direct API access for any application"
            />
            <IntegrationCard
              title="MCP"
              description="Model Context Protocol for AI systems"
            />
            <IntegrationCard
              title="n8n"
              description="Workflow automation and pipelines"
            />
          </div>
        </section>

        <section className="max-w-2xl mx-auto py-20 text-center">
          <h2 className="text-4xl font-bold mb-6">Ready to improve?</h2>
          <p className="text-xl text-gray-600 mb-8">
            Start versioning and improving your prompts today.
          </p>
          <Link
            to="/login"
            className="inline-block px-8 py-4 bg-gray-900 text-white text-lg rounded-lg font-semibold hover:bg-gray-800 transition-colors shadow-lg"
          >
            Get Started Free
          </Link>
        </section>
      </main>

      <footer className="border-t border-gray-200 py-12">
        <div className="container mx-auto px-6 text-center text-gray-600">
          <p>© 2026 Prompt Kaizen. Built for continuous improvement.</p>
        </div>
      </footer>
    </div>
  );
}

function FeatureCard({
  title,
  description,
  icon,
}: {
  title: string;
  description: string;
  icon: string;
}) {
  return (
    <div className="bg-white rounded-xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-shadow">
      <div className="text-4xl mb-4">{icon}</div>
      <h3 className="text-xl font-bold mb-3">{title}</h3>
      <p className="text-gray-600">{description}</p>
    </div>
  );
}

function IntegrationCard({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <div className="bg-gray-50 rounded-lg p-6 border border-gray-200">
      <h3 className="font-bold text-lg mb-2">{title}</h3>
      <p className="text-sm text-gray-600">{description}</p>
    </div>
  );
}
