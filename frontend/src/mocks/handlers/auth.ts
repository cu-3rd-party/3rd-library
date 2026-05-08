import { http, HttpResponse } from "msw";

type AuthUserRecord = {
  email: string;
  username: string;
  password: string;
  bio: string;
  image: string | null;
};

type LoginUserRequest = {
  user?: {
    email?: string;
    password?: string;
  };
};

type NewUserRequest = {
  user?: {
    username?: string;
    email?: string;
    password?: string;
  };
};

type UpdateCurrentUserRequest = {
  user?: {
    email?: string;
    username?: string;
    bio?: string;
    image?: string | null;
  };
};

const resolveApiBaseUrl = () =>
  import.meta.env.MOCK_API_URL ||
  import.meta.env.VITE_API_URL ||
  globalThis.location.origin;

const authUsersDb: AuthUserRecord[] = [
  {
    email: "demo@3rd-library.local",
    username: "demo",
    password: "password123",
    bio: "",
    image: null,
  },
];

const toToken = (email: string) => {
  const encodedEmail = btoa(email);
  return `mock-token-${encodedEmail}`;
};

const toUserResponse = (record: AuthUserRecord) => ({
  user: {
    email: record.email,
    username: record.username,
    token: toToken(record.email),
    bio: record.bio,
    image: record.image,
  },
});

const resolveUserFromRequest = (request: Request) => {
  const authHeader = request.headers.get("Authorization")?.trim() || "";
  const token = authHeader
    .replace(/^Token\s+/i, "")
    .replace(/^Bearer\s+/i, "")
    .trim();

  if (!token) return null;

  return authUsersDb.find((item) => toToken(item.email) === token) || null;
};

const unprocessableEntity = (field: string, message: string) =>
  HttpResponse.json(
    {
      errors: {
        [field]: [message],
      },
    },
    { status: 422 },
  );

export const authHandlers = [
  http.post(
    new URL("/api/users", resolveApiBaseUrl()).toString(),
    async ({ request }) => {
      const payload = (await request.json()) as NewUserRequest;
      const username = payload.user?.username?.trim() || "";
      const email = payload.user?.email?.trim() || "";
      const password = payload.user?.password || "";

      if (!username) return unprocessableEntity("username", "can't be blank");
      if (!email) return unprocessableEntity("email", "can't be blank");
      if (!password) return unprocessableEntity("password", "can't be blank");

      const normalizedEmail = email.toLowerCase();
      const normalizedUsername = username.toLowerCase();
      const isEmailTaken = authUsersDb.some(
        (item) => item.email.toLowerCase() === normalizedEmail,
      );
      if (isEmailTaken) return unprocessableEntity("email", "email taken");

      const isUsernameTaken = authUsersDb.some(
        (item) => item.username.toLowerCase() === normalizedUsername,
      );
      if (isUsernameTaken)
        return unprocessableEntity("username", "username taken");

      const newUser: AuthUserRecord = {
        email,
        username,
        password,
        bio: "",
        image: null,
      };
      authUsersDb.push(newUser);

      return HttpResponse.json(toUserResponse(newUser));
    },
  ),

  http.post(
    new URL("/api/users/login", resolveApiBaseUrl()).toString(),
    async ({ request }) => {
      const payload = (await request.json()) as LoginUserRequest;
      const email = payload.user?.email?.trim() || "";
      const password = payload.user?.password || "";

      console.log("SOMETHING")

      if (!email) return unprocessableEntity("email", "can't be blank");
      if (!password) return unprocessableEntity("password", "can't be blank");

      const user = authUsersDb.find(
        (item) => item.email.toLowerCase() === email.toLowerCase(),
      );
      if (!user) return unprocessableEntity("email", "does not exist");

      if (user.password !== password) {
        return new HttpResponse(null, {
          status: 401,
          headers: {
            "WWW-Authenticate": "Token",
          },
        });
      }

      return HttpResponse.json(toUserResponse(user));
    },
  ),

  http.get(new URL("/api/user", resolveApiBaseUrl()).toString(), ({ request }) => {
    const user = resolveUserFromRequest(request);
    if (!user) {
      return new HttpResponse(null, {
        status: 401,
        headers: {
          "WWW-Authenticate": "Token",
        },
      });
    }

    return HttpResponse.json(toUserResponse(user));
  }),

  http.put(
    new URL("/api/user", resolveApiBaseUrl()).toString(),
    async ({ request }) => {
      const currentUser = resolveUserFromRequest(request);
      if (!currentUser) {
        return new HttpResponse(null, {
          status: 401,
          headers: {
            "WWW-Authenticate": "Token",
          },
        });
      }

      const payload = (await request.json()) as UpdateCurrentUserRequest;
      const nextEmail = payload.user?.email?.trim() || currentUser.email;
      const nextUsername = payload.user?.username?.trim() || currentUser.username;
      const nextBio = (payload.user?.bio || "").trim();
      const nextImage =
        typeof payload.user?.image === "string"
          ? payload.user.image
          : payload.user?.image === null
            ? null
            : currentUser.image;

      if (!nextEmail) return unprocessableEntity("email", "can't be blank");
      if (!nextUsername) return unprocessableEntity("username", "can't be blank");

      const normalizedEmail = nextEmail.toLowerCase();
      const normalizedUsername = nextUsername.toLowerCase();

      const hasEmailConflict = authUsersDb.some(
        (item) =>
          item !== currentUser && item.email.toLowerCase() === normalizedEmail,
      );
      if (hasEmailConflict) {
        return unprocessableEntity("email", "email taken");
      }

      const hasUsernameConflict = authUsersDb.some(
        (item) =>
          item !== currentUser &&
          item.username.toLowerCase() === normalizedUsername,
      );
      if (hasUsernameConflict) {
        return unprocessableEntity("username", "username taken");
      }

      currentUser.email = nextEmail;
      currentUser.username = nextUsername;
      currentUser.bio = nextBio;
      currentUser.image = nextImage;

      return HttpResponse.json(toUserResponse(currentUser));
    },
  ),
];
