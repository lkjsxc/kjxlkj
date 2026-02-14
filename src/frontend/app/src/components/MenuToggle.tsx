// UX-LAYOUT-02: menu toggle for compact screens (<1024px).
// Collapses/restores navigation panel.

interface Props {
  sidebarOpen: boolean;
  onToggle: () => void;
}

export default function MenuToggle({ sidebarOpen, onToggle }: Props) {
  return (
    <button
      type="button"
      className="menu-toggle"
      onClick={onToggle}
      aria-label={sidebarOpen ? 'Close navigation' : 'Open navigation'}
      aria-expanded={sidebarOpen}
    >
      {sidebarOpen ? '✕' : '☰'}
    </button>
  );
}
