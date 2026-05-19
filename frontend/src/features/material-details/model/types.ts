import { Material } from "@/entities/material/model";
import { MaterialSubmissionFile } from "@/entities/submission/model";

export type MaterialDetailsResponse = Material & {
  authorImage?: string | null;
  files: MaterialSubmissionFile[];
  submittedAt?: string;
  publishedAt?: string;
};
