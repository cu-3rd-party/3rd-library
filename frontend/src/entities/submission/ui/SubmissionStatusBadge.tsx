import { cn } from "@/shared/lib";
import { Badge } from "@/shared/ui";

import { SUBMISSION_STATUS_UI } from "../lib/statusUI";
import { SubmissionStatus } from "../model";

type SubmissionStatusBadgeProps = {
  status: SubmissionStatus;
  className?: string;
};

export const SubmissionStatusBadge = ({
  status,
  className,
}: SubmissionStatusBadgeProps) => {
  const statusUi = SUBMISSION_STATUS_UI[status];

  return (
    <Badge
      variant="secondary"
      className={cn(statusUi.badgeClassName, className)}
    >
      {statusUi.label}
    </Badge>
  );
};
