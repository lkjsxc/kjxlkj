/**
 * Lazy-loaded wrappers for heavyweight panels.
 * Per IMP-BACKLOG-FE-03: React lazy-split for review panel.
 * Reduces initial bundle size by deferring librarian review code.
 */
import { lazy, Suspense, type ComponentProps } from "react";

const LazyLibrarianReview = lazy(() =>
  import("./LibrarianReview").then((m) => ({ default: m.LibrarianReview })),
);

const LazyJobsPanel = lazy(() =>
  import("./JobsPanel").then((m) => ({ default: m.JobsPanel })),
);

function LoadingFallback() {
  return <div style={{ padding: "1rem", color: "#999" }}>Loading…</div>;
}

export function LibrarianReviewLazy(
  props: ComponentProps<typeof LazyLibrarianReview>,
) {
  return (
    <Suspense fallback={<LoadingFallback />}>
      <LazyLibrarianReview {...props} />
    </Suspense>
  );
}

export function JobsPanelLazy(
  props: ComponentProps<typeof LazyJobsPanel>,
) {
  return (
    <Suspense fallback={<LoadingFallback />}>
      <LazyJobsPanel {...props} />
    </Suspense>
  );
}
