import { http, HttpResponse } from "msw";

import { MOCK_USER } from "@/mocks";
import { Course, Difficulty, MaterialType, Subject } from "@/models/material";
import {
  MaterialSubmission,
  MaterialSubmissionFile,
  SubmissionStatus,
} from "@/models/materialSubmission";

import {
  ensureUserExists,
  getSubmissionCounters,
  getSubmissionsDb,
  setSubmissionsDb,
  upsertSubmission,
} from "./db";

type PaginatedSubmissionsResponse = {
  items: MaterialSubmission[];
  page: number;
  limit: number;
  total: number;
};

type PaginatedModerationResponse = PaginatedSubmissionsResponse & {
  counters: ReturnType<typeof getSubmissionCounters>;
};

type ModerationDecisionRequest =
  | {
      action: "approve";
    }
  | {
      action: "reject";
      moderatorComment: string;
    };

const DEFAULT_PAGE = 1;
const DEFAULT_LIMIT = 20;

const resolveApiBaseUrl = () =>
  import.meta.env.MOCK_API_URL ||
  import.meta.env.VITE_API_URL ||
  globalThis.location.origin;

const formatDisplayDate = (date: Date) =>
  new Intl.DateTimeFormat("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(date);

const toPositiveInt = (value: string | null, fallback: number) => {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) return fallback;
  return Math.floor(parsed);
};

const parseStatusFilter = (
  value: string | null,
): SubmissionStatus | "all" | null => {
  if (!value) return null;
  if (
    value === "draft" ||
    value === "pending_review" ||
    value === "rejected" ||
    value === "approved" ||
    value === "all"
  ) {
    return value;
  }

  return null;
};

const paginate = <T>(items: T[], page: number, limit: number) => {
  const start = (page - 1) * limit;
  return items.slice(start, start + limit);
};

const parseStringList = (formData: FormData, key: string) =>
  formData
    .getAll(key)
    .flatMap((value) => String(value).split(","))
    .map((value) => value.trim())
    .filter(Boolean);

const parseKeepFileIds = (formData: FormData) =>
  formData
    .getAll("keepFileIds")
    .map((value) => String(value).trim())
    .filter(Boolean);

const bytesToBase64 = (bytes: Uint8Array) => {
  let binary = "";
  const chunkSize = 0x8000;

  for (let index = 0; index < bytes.length; index += chunkSize) {
    const chunk = bytes.subarray(index, index + chunkSize);
    binary += String.fromCharCode(...chunk);
  }

  return btoa(binary);
};

const toSubmissionFiles = async (
  formData: FormData,
  currentFiles: MaterialSubmissionFile[] = [],
): Promise<MaterialSubmissionFile[]> => {
  const keepFileIds = new Set(parseKeepFileIds(formData));
  const existingFiles = currentFiles.filter(
    (file) => file.id && keepFileIds.has(file.id),
  );
  const uploadedFilesRaw = formData
    .getAll("files")
    .filter((item): item is File => item instanceof File);

  const uploadedFiles = await Promise.all(
    uploadedFilesRaw.map(async (file) => {
      let url: string | undefined;

      try {
        const bytes = new Uint8Array(await file.arrayBuffer());
        const base64 = bytesToBase64(bytes);
        const mimeType = file.type || "application/octet-stream";
        url = `data:${mimeType};base64,${base64}`;
      } catch {
        url = undefined;
      }

      return {
        id: crypto.randomUUID(),
        name: file.name,
        sizeBytes: file.size,
        extension: file.name.split(".").pop()?.toLowerCase() || "",
        mimeType: file.type || undefined,
        url,
      };
    }),
  );

  return [...existingFiles, ...uploadedFiles];
};

const getRequiredString = (formData: FormData, key: string) =>
  String(formData.get(key) || "").trim();

const getOptionalString = (formData: FormData, key: string) =>
  String(formData.get(key) || "").trim();

const badRequest = (message: string) =>
  HttpResponse.json(
    {
      code: "validation_error",
      message,
    },
    { status: 400 },
  );

const findSubmissionById = (submissionId: string) =>
  getSubmissionsDb().find((item) => item.id === submissionId) || null;

const updateSubmissionInDb = (nextSubmission: MaterialSubmission) => {
  const next = getSubmissionsDb().map((item) =>
    item.id === nextSubmission.id ? nextSubmission : item,
  );
  setSubmissionsDb(next);
};

export const submissionsHandlers = [
  http.get(
    new URL("/materials/submissions", resolveApiBaseUrl()).toString(),
    ({ request }) => {
      const url = new URL(request.url);
      const page = toPositiveInt(url.searchParams.get("page"), DEFAULT_PAGE);
      const limit = toPositiveInt(url.searchParams.get("limit"), DEFAULT_LIMIT);
      const statusFilter = parseStatusFilter(url.searchParams.get("status"));
      const authorId = url.searchParams.get("authorId") || MOCK_USER.id;

      const ownSubmissions = getSubmissionsDb()
        .filter((submission) => submission.material.authorId === authorId)
        .filter((submission) =>
          statusFilter && statusFilter !== "all"
            ? submission.status === statusFilter
            : true,
        )
        .sort(
          (first, second) =>
            new Date(second.updatedAt).getTime() -
            new Date(first.updatedAt).getTime(),
        );

      const response: PaginatedSubmissionsResponse = {
        items: paginate(ownSubmissions, page, limit),
        page,
        limit,
        total: ownSubmissions.length,
      };

      return HttpResponse.json(response);
    },
  ),

  http.get(
    new URL(
      "/materials/submissions/:submissionId",
      resolveApiBaseUrl(),
    ).toString(),
    ({ params }) => {
      const submissionId = String(params.submissionId || "");
      const submission = findSubmissionById(submissionId);

      if (!submission) {
        return HttpResponse.json(
          { code: "not_found", message: "Submission not found" },
          { status: 404 },
        );
      }

      return HttpResponse.json(submission);
    },
  ),

  http.post(
    new URL("/materials/submissions", resolveApiBaseUrl()).toString(),
    async ({ request }) => {
      const formData = await request.formData();
      const title = getRequiredString(formData, "title");
      const description = getOptionalString(formData, "description");
      const type = getRequiredString(formData, "type") as MaterialType;
      const difficulty = getRequiredString(
        formData,
        "difficulty",
      ) as Difficulty;
      const courses = parseStringList(formData, "courses") as Course[];
      const subjects = parseStringList(formData, "subjects") as Subject[];
      const files = await toSubmissionFiles(formData);
      const authorId = getOptionalString(formData, "authorId") || MOCK_USER.id;
      const authorName =
        getOptionalString(formData, "authorName") || MOCK_USER.name;

      if (!title) return badRequest("Title is required");
      if (courses.length === 0)
        return badRequest("At least one course is required");
      if (subjects.length === 0)
        return badRequest("At least one subject is required");
      if (!type) return badRequest("Type is required");
      if (!difficulty) return badRequest("Difficulty is required");
      if (files.length === 0)
        return badRequest("At least one file is required");

      const now = new Date();
      const nowIso = now.toISOString();

      const submission: MaterialSubmission = {
        id: crypto.randomUUID(),
        material: {
          id: crypto.randomUUID(),
          authorId,
          authorName,
          title,
          description,
          courses: [...courses].sort(),
          subjects: [...subjects].sort(),
          type,
          difficulty,
          pubDate: formatDisplayDate(now),
        },
        files,
        file: files[0] || null,
        status: "pending_review",
        moderatorComment: "",
        createdAt: nowIso,
        updatedAt: nowIso,
        submittedAt: nowIso,
      };

      upsertSubmission(submission);
      ensureUserExists({
        id: authorId,
        name: authorName,
        bio: "",
        verified: true,
      });

      return HttpResponse.json(submission, { status: 201 });
    },
  ),

  http.patch(
    new URL(
      "/materials/submissions/:submissionId",
      resolveApiBaseUrl(),
    ).toString(),
    async ({ request, params }) => {
      const submissionId = String(params.submissionId || "");
      const current = findSubmissionById(submissionId);

      if (!current) {
        return HttpResponse.json(
          { code: "not_found", message: "Submission not found" },
          { status: 404 },
        );
      }

      const formData = await request.formData();
      const nextFiles = await toSubmissionFiles(formData, current.files);
      const now = new Date();
      const nowIso = now.toISOString();

      const title =
        getOptionalString(formData, "title") || current.material.title;
      const description =
        getOptionalString(formData, "description") ||
        current.material.description;
      const type =
        (getOptionalString(formData, "type") as MaterialType) ||
        current.material.type;
      const difficulty =
        (getOptionalString(formData, "difficulty") as Difficulty) ||
        current.material.difficulty;

      const coursesInput = parseStringList(formData, "courses") as Course[];
      const subjectsInput = parseStringList(formData, "subjects") as Subject[];
      const courses = coursesInput.length
        ? coursesInput
        : current.material.courses;
      const subjects = subjectsInput.length
        ? subjectsInput
        : current.material.subjects;
      const files = nextFiles.length ? nextFiles : current.files;

      const updatedSubmission: MaterialSubmission = {
        ...current,
        material: {
          ...current.material,
          title,
          description,
          courses: [...courses].sort(),
          subjects: [...subjects].sort(),
          type,
          difficulty,
        },
        files,
        file: files[0] || null,
        status: "pending_review",
        moderatorComment: "",
        updatedAt: nowIso,
        submittedAt: nowIso,
      };

      updateSubmissionInDb(updatedSubmission);

      return HttpResponse.json(updatedSubmission);
    },
  ),

  http.get(
    new URL("/moderation/submissions", resolveApiBaseUrl()).toString(),
    ({ request }) => {
      const url = new URL(request.url);
      const page = toPositiveInt(url.searchParams.get("page"), DEFAULT_PAGE);
      const limit = toPositiveInt(url.searchParams.get("limit"), DEFAULT_LIMIT);
      const statusFilter =
        parseStatusFilter(url.searchParams.get("status")) || "pending_review";

      const all = getSubmissionsDb().sort(
        (first, second) =>
          new Date(second.updatedAt).getTime() -
          new Date(first.updatedAt).getTime(),
      );
      const filtered = all.filter((submission) =>
        statusFilter === "all" ? true : submission.status === statusFilter,
      );

      const response: PaginatedModerationResponse = {
        items: paginate(filtered, page, limit),
        page,
        limit,
        total: filtered.length,
        counters: getSubmissionCounters(all),
      };

      return HttpResponse.json(response);
    },
  ),

  http.post(
    new URL(
      "/moderation/submissions/:submissionId/decision",
      resolveApiBaseUrl(),
    ).toString(),
    async ({ request, params }) => {
      const submissionId = String(params.submissionId || "");
      const current = findSubmissionById(submissionId);

      if (!current) {
        return HttpResponse.json(
          { code: "not_found", message: "Submission not found" },
          { status: 404 },
        );
      }

      const payload = (await request.json()) as ModerationDecisionRequest;
      const now = new Date();
      const nowIso = now.toISOString();

      if (payload.action === "approve") {
        const approvedSubmission: MaterialSubmission = {
          ...current,
          status: "approved",
          moderatorComment: "",
          updatedAt: nowIso,
          reviewedAt: nowIso,
          publishedAt: nowIso,
          material: {
            ...current.material,
            pubDate: formatDisplayDate(now),
          },
        };

        updateSubmissionInDb(approvedSubmission);
        return HttpResponse.json(approvedSubmission);
      }

      if (!payload.moderatorComment?.trim()) {
        return badRequest("Moderator comment is required for reject action");
      }

      const rejectedSubmission: MaterialSubmission = {
        ...current,
        status: "rejected",
        moderatorComment: payload.moderatorComment.trim(),
        updatedAt: nowIso,
        reviewedAt: nowIso,
      };

      updateSubmissionInDb(rejectedSubmission);
      return HttpResponse.json(rejectedSubmission);
    },
  ),
];
