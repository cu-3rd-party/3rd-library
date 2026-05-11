const SKELETON_ITEMS = Array.from({ length: 8 }, (_, index) => index);

export const ModerationSubmissionsSkeleton = () => (
  <div className="grid grid-cols-2 gap-4 md:grid-cols-2 xl:grid-cols-4">
    {SKELETON_ITEMS.map((key) => (
      <div
        key={key}
        className="animate-pulse rounded-xl border border-border bg-card p-4"
      >
        <div className="aspect-[16/9] rounded-lg bg-muted/70" />
        <div className="mt-4 space-y-2">
          <div className="h-4 w-3/4 rounded bg-muted/70" />
          <div className="h-4 w-1/2 rounded bg-muted/70" />
          <div className="h-3 w-2/5 rounded bg-muted/60" />
        </div>
      </div>
    ))}
  </div>
);
