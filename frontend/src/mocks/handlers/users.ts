import { http, HttpResponse } from "msw";

import { MOCK_USER } from "@/mocks";
import { Material, User } from "@/models";

import { getUserById, getUserPublishedMaterials, getUsersDb } from "./db";

type PaginatedUsersResponse = {
  items: User[];
  page: number;
  limit: number;
  total: number;
};

type UserWithMaterialsResponse = {
  user: User;
  materials: {
    items: Material[];
    page: number;
    limit: number;
    total: number;
  };
};

const DEFAULT_PAGE = 1;
const DEFAULT_LIMIT = 20;

const toPositiveInt = (value: string | null, fallback: number) => {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) return fallback;
  return Math.floor(parsed);
};

const resolveApiBaseUrl = () =>
  import.meta.env.MOCK_API_URL ||
  import.meta.env.VITE_API_URL ||
  globalThis.location.origin;

export const usersHandlers = [
  http.get(new URL("/users", resolveApiBaseUrl()).toString(), ({ request }) => {
    const url = new URL(request.url);
    const search = url.searchParams.get("search")?.trim().toLowerCase() || "";
    const page = toPositiveInt(url.searchParams.get("page"), DEFAULT_PAGE);
    const limit = toPositiveInt(url.searchParams.get("limit"), DEFAULT_LIMIT);

    const filteredUsers = getUsersDb().filter((user) =>
      user.name.toLowerCase().includes(search),
    );

    const start = (page - 1) * limit;
    const items = filteredUsers.slice(start, start + limit);

    const response: PaginatedUsersResponse = {
      items,
      page,
      limit,
      total: filteredUsers.length,
    };

    return HttpResponse.json(response);
  }),

  http.get(
    new URL("/users/:userId", resolveApiBaseUrl()).toString(),
    ({ request, params }) => {
      const url = new URL(request.url);
      const userId = String(params.userId || "");
      const page = toPositiveInt(url.searchParams.get("page"), DEFAULT_PAGE);
      const limit = toPositiveInt(url.searchParams.get("limit"), DEFAULT_LIMIT);
      const user = getUserById(userId);

      if (!user) {
        return HttpResponse.json(
          { code: "not_found", message: "User not found" },
          { status: 404 },
        );
      }

      const allMaterials = getUserPublishedMaterials(userId);
      const start = (page - 1) * limit;
      const items = allMaterials.slice(start, start + limit);

      const response: UserWithMaterialsResponse = {
        user,
        materials: {
          items,
          page,
          limit,
          total: allMaterials.length,
        },
      };

      return HttpResponse.json(response);
    },
  ),

  http.get(new URL("/users/me", resolveApiBaseUrl()).toString(), () =>
    HttpResponse.json(MOCK_USER),
  ),
];
