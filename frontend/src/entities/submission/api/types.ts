import { LibraryMaterial, LibraryMaterialFile } from "@/entities/material/api";

export type LibrarySubmission = {
  id: string;
  material: LibraryMaterial;
  files: LibraryMaterialFile[];
  status: string;
  moderator_comment: string;
  created_at: string;
  updated_at: string;
  submitted_at?: string | null;
  reviewed_at?: string | null;
  published_at?: string | null;
};

export type LibraryPaginatedSubmissionsResponse = {
  items: LibrarySubmission[];
  page: number;
  limit: number;
  total: number;
};

export type LibraryModerationCounters = {
  all: number;
  draft: number;
  pending_review: number;
  rejected: number;
  approved: number;
};

export type LibraryModerationResponse = {
  items: LibrarySubmission[];
  page: number;
  limit: number;
  total: number;
  counters: LibraryModerationCounters;
};
