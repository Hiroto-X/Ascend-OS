import type { PropsWithChildren } from "react";
import { NavLink } from "react-router";
import { useShellStore } from "../state/shell";

const navItems = [
  { to: "/home", label: "Home" },
  { to: "/settings", label: "Settings" },
];

export function AppShell({ children }: PropsWithChildren) {
  const { isNavigationOpen, setNavigationOpen } = useShellStore();

  return (
    <div className="app-shell">
      <header className="topbar">
        <button
          type="button"
          className="navigation-toggle"
          aria-expanded={isNavigationOpen}
          aria-controls="primary-navigation"
          onClick={() => setNavigationOpen(!isNavigationOpen)}
        >
          Navigation
        </button>
        <p className="product-name">Ascend OS</p>
      </header>
      <div className="shell-body">
        <nav
          id="primary-navigation"
          className="sidebar"
          aria-label="Primary navigation"
          hidden={!isNavigationOpen}
        >
          {navItems.map((item) => (
            <NavLink key={item.to} to={item.to} className="navigation-link">
              {item.label}
            </NavLink>
          ))}
        </nav>
        <main className="content">{children}</main>
      </div>
    </div>
  );
}
