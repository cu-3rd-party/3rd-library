import { Badge } from "@/components/ui/badge";
import { SUBMISSION_STATUS_UI } from "@/constants";
import { cn } from "@/lib/utils";
import { SubmissionStatus } from "@/models";

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
