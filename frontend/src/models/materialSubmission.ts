import {
  Course,
  Difficulty,
  Material,
  MaterialType,
  Subject,
} from "./material";
import { UUID } from "./types";

export type SubmissionStatus =
  | "draft"
  | "pending_review"
  | "rejected"
  | "approved";

export interface MaterialSubmissionFile {
  id?: UUID;
  name: string;
  sizeBytes: number;
  extension: string;
  mimeType?: string;
  url?: string;
}

export type MaterialUploadValue = File | MaterialSubmissionFile;

export interface MaterialSubmission {
  id: UUID;
  material: Material;
  files: MaterialSubmissionFile[];
  file?: MaterialSubmissionFile | null;
  status: SubmissionStatus;
  moderatorComment: string;
  createdAt: string;
  updatedAt: string;
  submittedAt?: string;
  reviewedAt?: string;
  publishedAt?: string;
}

export interface MaterialSubmissionInput {
  id?: UUID;
  authorId: UUID;
  authorName: string;
  title: string;
  description: string;
  courses: Course[];
  subjects: Subject[];
  type: MaterialType;
  difficulty: Difficulty;
  files: MaterialUploadValue[];
  file?: MaterialUploadValue | null;
}
