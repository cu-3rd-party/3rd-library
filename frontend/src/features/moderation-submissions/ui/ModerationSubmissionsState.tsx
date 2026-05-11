import { ReactNode } from "react";

import { ModerationSubmissionsSkeleton } from "./ModerationSubmissionsSkeleton";

type ModerationSubmissionsStateProps = {
  isLoading: boolean;
  isError: boolean;
  errorMessage: string;
  children: ReactNode;
};

export const ModerationSubmissionsState = ({
  isLoading,
  isError,
  errorMessage,
  children,
}: ModerationSubmissionsStateProps) => {
  if (isLoading) {
    return <ModerationSubmissionsSkeleton />;
  }

  if (isError) {
    return (
      <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-destructive">
        {errorMessage || "Не удалось загрузить список заявок"}
      </div>
    );
  }

  return <>{children}</>;
};
