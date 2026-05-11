import { fetchJsonWithAuth } from "@/entities/session/api";
import { LibrarySubmission } from "@/entities/submission/api";
import { resolveApiUrl } from "@/shared/api";

export type ModerationDecisionAction = "approve" | "reject";

type SendModerationDecisionParams = {
  submissionId: string;
  action: ModerationDecisionAction;
  moderatorComment?: string;
};

export const sendModerationDecision = ({
  submissionId,
  action,
  moderatorComment,
}: SendModerationDecisionParams) =>
  fetchJsonWithAuth<LibrarySubmission>(
    resolveApiUrl(
      `/api/moderation/submissions/${encodeURIComponent(submissionId)}/decision`,
    ),
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body:
        action === "reject"
          ? JSON.stringify({
              action,
              moderator_comment: moderatorComment?.trim() || "",
            })
          : JSON.stringify({ action }),
    },
  );
