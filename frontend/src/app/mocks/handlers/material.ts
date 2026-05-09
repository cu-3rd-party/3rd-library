import { http, HttpResponse } from "msw";

import {
  Course,
  Difficulty,
  Material,
  MaterialType,
  Subject,
} from "@/entities/material/model";

import {
  findApprovedSubmissionByMaterialId,
  getPublishedMaterials,
} from "../db";

type PaginatedMaterialsResponse = {
  items: Material[];
  page: number;
  limit: number;
  total: number;
};

type SortBy = "date" | "title" | "subject";
type SortOrder = "increasing" | "decreasing";

const DEFAULT_PAGE = 1;
const DEFAULT_LIMIT = 20;
const DEFAULT_SORT_BY: SortBy = "date";
const DEFAULT_ORDER: SortOrder = "decreasing";

const toPositiveInt = (value: string | null, fallback: number) => {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) return fallback;
  return Math.floor(parsed);
};

const parseCsv = <T extends string>(value: string | null): T[] => {
  if (!value) return [];

  return value
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean) as T[];
};

const parseSortBy = (value: string | null): SortBy =>
  value === "title" || value === "subject" || value === "date"
    ? value
    : DEFAULT_SORT_BY;

const parseOrder = (value: string | null): SortOrder =>
  value === "increasing" || value === "decreasing" ? value : DEFAULT_ORDER;

const parseDisplayDate = (value: string) => {
  const [day, month, year] = value.split(".");
  const timestamp = new Date(`${year}-${month}-${day}`).getTime();
  return Number.isNaN(timestamp) ? 0 : timestamp;
};

const resolveApiBaseUrl = () =>
  import.meta.env.MOCK_API_URL ||
  import.meta.env.VITE_API_URL ||
  globalThis.location.origin;

export const materialsHandlers = [
  http.get(
    new URL("/materials", resolveApiBaseUrl()).toString(),
    ({ request }) => {
      const url = new URL(request.url);
      const search = url.searchParams.get("search")?.trim().toLowerCase() || "";
      const courses = parseCsv<Course>(url.searchParams.get("courses"));
      const subjects = parseCsv<Subject>(url.searchParams.get("subjects"));
      const types = parseCsv<MaterialType>(url.searchParams.get("types"));
      const difficulties = parseCsv<Difficulty>(
        url.searchParams.get("difficulties"),
      );
      const sortBy = parseSortBy(url.searchParams.get("sortBy"));
      const order = parseOrder(url.searchParams.get("order"));
      const page = toPositiveInt(url.searchParams.get("page"), DEFAULT_PAGE);
      const limit = toPositiveInt(url.searchParams.get("limit"), DEFAULT_LIMIT);

      const filtered = getPublishedMaterials().filter((material) => {
        const matchesSearch = material.title.toLowerCase().includes(search);
        if (!matchesSearch) return false;

        const matchesCourses =
          courses.length === 0 ||
          courses.some((course) => material.courses.includes(course));
        if (!matchesCourses) return false;

        const matchesSubjects =
          subjects.length === 0 ||
          subjects.some((subject) => material.subjects.includes(subject));
        if (!matchesSubjects) return false;

        const matchesTypes =
          types.length === 0 || types.includes(material.type);
        if (!matchesTypes) return false;

        const matchesDifficulties =
          difficulties.length === 0 ||
          difficulties.includes(material.difficulty);
        return matchesDifficulties;
      });

      const sorted = [...filtered].sort((first, second) => {
        let comparison = 0;

        if (sortBy === "date") {
          comparison =
            parseDisplayDate(first.pubDate) - parseDisplayDate(second.pubDate);
        } else if (sortBy === "title") {
          comparison = first.title.localeCompare(second.title);
        } else {
          comparison = (first.subjects[0] || "").localeCompare(
            second.subjects[0] || "",
          );
        }

        return order === "increasing" ? comparison : -comparison;
      });

      const start = (page - 1) * limit;
      const items = sorted.slice(start, start + limit);

      const response: PaginatedMaterialsResponse = {
        items,
        page,
        limit,
        total: sorted.length,
      };

      return HttpResponse.json(response);
    },
  ),

  http.get(
    new URL("/materials/:materialId", resolveApiBaseUrl()).toString(),
    ({ params }) => {
      const materialId = String(params.materialId || "");
      const submission = findApprovedSubmissionByMaterialId(materialId);

      if (!submission) {
        return HttpResponse.json(
          { code: "not_found", message: "Material not found" },
          { status: 404 },
        );
      }

      return HttpResponse.json({
        ...submission.material,
        files: submission.files,
        publishedAt: submission.publishedAt,
        submittedAt: submission.submittedAt,
      });
    },
  ),
];
