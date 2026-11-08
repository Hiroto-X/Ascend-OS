import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { HashRouter, Navigate, Route, Routes } from "react-router";
import { SettingsPage } from "./features/settings/SettingsPage";
import { AppShell } from "./shell/AppShell";
import "./App.css";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

function PlaceholderPage({ title }: { title: string }) {
  return (
    <section aria-labelledby="placeholder-heading" className="placeholder-page">
      <p className="eyebrow">Ascend OS</p>
      <h1 id="placeholder-heading">{title}</h1>
      <p>This workspace will be available in a later roadmap phase.</p>
    </section>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <HashRouter>
        <AppShell>
          <Routes>
            <Route path="/home" element={<PlaceholderPage title="Home" />} />
            <Route path="/settings" element={<SettingsPage />} />
            <Route path="*" element={<Navigate to="/settings" replace />} />
          </Routes>
        </AppShell>
      </HashRouter>
    </QueryClientProvider>
  );
}

export default App;
