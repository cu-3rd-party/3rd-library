import { MaterialCard } from "@/common/molecules";
import { MaterialSubmission } from "@/models";

type ModerationSubmissionGridProps = {
  submissions: MaterialSubmission[];
  onSubmissionClick: (submission: MaterialSubmission) => void;
};

export const ModerationSubmissionGrid = ({
  submissions,
  onSubmissionClick,
}: ModerationSubmissionGridProps) => {
  if (!submissions.length) {
    return (
      <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-muted-foreground">
        По выбранному статусу заявок нет.
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 gap-4 md:grid-cols-2 xl:grid-cols-4">
      {submissions.map((submission) => (
        <article key={submission.id}>
          <MaterialCard
            material={submission.material}
            onClick={() => onSubmissionClick(submission)}
            showAuthor
            className="h-full"
          />
        </article>
      ))}
    </div>
  );
};
