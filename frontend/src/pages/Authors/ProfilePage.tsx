import { LoaderCircle, Pencil, Upload, User } from "lucide-react";
import { ChangeEvent, useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

import { MaterialsSection } from "@/common/organisms";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { AUTHORS_PREFIX } from "@/constants";
import { fetchJson, resolveApiUrl } from "@/lib/api";
import { updateCurrentUser } from "@/lib/authApi";
import {
  getCurrentAuthUser,
  persistCurrentAuthUser,
  StoredAuthUser,
} from "@/lib/currentUser";
import {
  mapArticleToMaterial,
  mapProfileToUser,
  RealWorldArticlesResponse,
  RealWorldProfileResponse,
} from "@/lib/materialsApi";
import { MOCK_AUTHORS, MOCK_SUBMISSIONS, MOCK_USER } from "@/mocks/mockData";
import { Material, User as UserModel } from "@/models";

type UserProfileResponse = {
  user: UserModel;
  materials: {
    items: Material[];
    page: number;
    limit: number;
    total: number;
  };
};

const MAX_BIO_LENGTH = 2_000;
const UUID_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

const decodeRouteUserId = (value?: string) => {
  const fallback = (value || "").trim();

  try {
    return decodeURIComponent(fallback).trim();
  } catch {
    return fallback;
  }
};

const readFileAsDataUrl = (file: File) =>
  new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result !== "string") {
        reject(new Error("Failed to read image as data URL."));
        return;
      }

      resolve(reader.result);
    };
    reader.onerror = () => reject(new Error("Failed to read image file."));
    reader.readAsDataURL(file);
  });

const isSameAuthUser = (
  first: StoredAuthUser | null,
  second: StoredAuthUser,
) =>
  first
    ? first.email === second.email &&
      first.username === second.username &&
      first.bio === second.bio &&
      first.image === second.image
    : false;

const getFallbackProfile = (userId: string): UserProfileResponse | null => {
  const fallbackUser = [MOCK_USER, ...MOCK_AUTHORS].find(
    (user) => user.id === userId,
  );
  if (!fallbackUser) return null;

  const userMaterials = MOCK_SUBMISSIONS.filter(
    (submission) =>
      submission.status === "approved" &&
      submission.material.authorId === userId,
  ).map((submission) => submission.material);

  return {
    user: fallbackUser,
    materials: {
      items: userMaterials,
      page: 1,
      limit: 20,
      total: userMaterials.length,
    },
  };
};

const ProfilePage = () => {
  const navigate = useNavigate();
  const { id } = useParams();
  const routeUserId = decodeRouteUserId(id);
  const [imageError, setImageError] = useState(false);
  const [profile, setProfile] = useState<UserProfileResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [currentAuthUser, setCurrentAuthUser] = useState<StoredAuthUser | null>(
    () => getCurrentAuthUser(),
  );
  const [isEditOpen, setIsEditOpen] = useState(false);
  const [nicknameDraft, setNicknameDraft] = useState("");
  const [bioDraft, setBioDraft] = useState("");
  const [avatarDraft, setAvatarDraft] = useState<string | null>(null);
  const [editError, setEditError] = useState("");
  const [isSavingProfile, setIsSavingProfile] = useState(false);
  const avatarInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const userId = routeUserId;
    if (!userId) {
      setIsLoading(false);
      setIsError(true);
      return;
    }

    const abortController = new AbortController();

    const fetchProfile = async () => {
      setIsLoading(true);
      setIsError(false);

      try {
        const encodedUserId = encodeURIComponent(userId);
        const [profilePayload, articlesPayload] = await Promise.all([
          fetchJson<RealWorldProfileResponse>(
            resolveApiUrl(`/api/profiles/${encodedUserId}`),
            { signal: abortController.signal },
          ),
          fetchJson<RealWorldArticlesResponse>(
            resolveApiUrl(`/api/articles?author=${encodedUserId}&limit=100`),
            { signal: abortController.signal },
          ),
        ]);

        const user = mapProfileToUser(profilePayload.profile);
        const isCurrentUserProfile =
          Boolean(currentAuthUser?.username) &&
          (userId === currentAuthUser?.username ||
            user.id === currentAuthUser?.username);

        if (isCurrentUserProfile) {
          const syncedAuthUser: StoredAuthUser = {
            email: currentAuthUser?.email || "",
            username: user.id,
            bio: user.bio,
            image: user.image || null,
          };

          if (!isSameAuthUser(currentAuthUser, syncedAuthUser)) {
            persistCurrentAuthUser(syncedAuthUser);
            setCurrentAuthUser(syncedAuthUser);
          }
        }

        const userMaterials =
          articlesPayload.articles.map(mapArticleToMaterial);

        setProfile({
          user,
          materials: {
            items: userMaterials,
            page: 1,
            limit: userMaterials.length,
            total: articlesPayload.articlesCount,
          },
        });
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          const fallbackProfile = getFallbackProfile(userId);
          if (fallbackProfile) {
            console.warn("[Profile] Falling back to local mock profile.");
            setProfile(fallbackProfile);
            setIsError(false);
            return;
          }

          if (currentAuthUser?.username && currentAuthUser.username === userId) {
            setProfile({
              user: {
                id: currentAuthUser.username,
                name: currentAuthUser.username,
                bio: currentAuthUser.bio,
                image: currentAuthUser.image,
                isEmailVerified: true,
                verified: true,
              },
              materials: {
                items: [],
                page: 1,
                limit: 20,
                total: 0,
              },
            });
            setIsError(false);
            return;
          }
        }

        console.error(error);
        setIsError(true);
      } finally {
        if (!abortController.signal.aborted) {
          setIsLoading(false);
        }
      }
    };

    fetchProfile();

    return () => abortController.abort();
  }, [currentAuthUser, routeUserId]);

  useEffect(() => {
    setImageError(false);
  }, [profile?.user.id, profile?.user.image]);

  if (isLoading) {
    return (
      <div className="container mx-auto max-w-screen-xl px-4 lg:px-8 py-10">
        <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-muted-foreground">
          Загружаем профиль...
        </div>
      </div>
    );
  }

  if (isError || !profile) {
    return (
      <div className="container mx-auto max-w-screen-xl px-4 lg:px-8 py-10">
        <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-destructive">
          Не удалось загрузить профиль
        </div>
      </div>
    );
  }

  const isOwnProfile =
    Boolean(currentAuthUser?.username) &&
    (routeUserId === currentAuthUser?.username ||
      profile.user.id === currentAuthUser?.username);
  const avatarSrc =
    profile.user.image ||
    (UUID_PATTERN.test(profile.user.id)
      ? `/avatars/${encodeURIComponent(profile.user.id)}.png`
      : null);

  const openEditDialog = () => {
    setNicknameDraft(profile.user.name);
    setBioDraft(profile.user.bio);
    setAvatarDraft(profile.user.image || null);
    setEditError("");
    setIsEditOpen(true);
  };

  const handleAvatarChange = (event: ChangeEvent<HTMLInputElement>) => {
    const selectedFile = event.target.files?.[0];
    event.target.value = "";
    if (!selectedFile) return;

    if (!selectedFile.type.startsWith("image/")) {
      setEditError("Можно загрузить только изображение.");
      return;
    }

    void readFileAsDataUrl(selectedFile)
      .then((dataUrl) => {
        setAvatarDraft(dataUrl);
        setEditError("");
      })
      .catch((error) => {
        console.error(error);
        setEditError("Не удалось прочитать изображение. Попробуйте другой файл.");
      });
  };

  const handleSaveProfile = async () => {
    if (!currentAuthUser) return;
    if (!currentAuthUser.email.trim()) {
      setEditError(
        "Не удалось определить email текущего пользователя. Выполните вход заново.",
      );
      return;
    }

    const nextUsername = nicknameDraft.trim();
    if (!nextUsername) {
      setEditError("Никнейм не может быть пустым.");
      return;
    }

    const nextBio = bioDraft.trim();
    if (nextBio.length > MAX_BIO_LENGTH) {
      setEditError(`Bio не должен превышать ${MAX_BIO_LENGTH} символов.`);
      return;
    }

    setIsSavingProfile(true);
    setEditError("");

    try {
      const payload = {
        email: currentAuthUser.email,
        username: nextUsername,
        bio: nextBio,
        image: avatarDraft,
      };
      const response = await updateCurrentUser(payload);
      const nextAuthUser: StoredAuthUser = {
        email: response.user.email,
        username: response.user.username,
        bio: response.user.bio || "",
        image: response.user.image,
      };

      globalThis.localStorage?.setItem(
        "accessToken",
        `Token ${response.user.token}`,
      );

      persistCurrentAuthUser(nextAuthUser);
      setCurrentAuthUser(nextAuthUser);
      setProfile((current) =>
        current
          ? {
              ...current,
              user: {
                ...current.user,
                id: nextAuthUser.username,
                name: nextAuthUser.username,
                bio: nextAuthUser.bio,
                image: nextAuthUser.image,
              },
            }
          : current,
      );
      setImageError(false);
      setIsEditOpen(false);
      navigate(`${AUTHORS_PREFIX}/${encodeURIComponent(nextAuthUser.username)}`, {
        replace: true,
      });
    } catch (error) {
      console.error(error);
      setEditError(
        error instanceof Error
          ? error.message
          : "Не удалось сохранить изменения профиля.",
      );
    } finally {
      setIsSavingProfile(false);
    }
  };

  return (
    <div className="container mx-auto max-w-screen-xl overflow-x-hidden px-4 py-8 lg:px-8 lg:py-10 space-y-8 lg:space-y-10">
      <div className="flex flex-col items-start gap-4 md:flex-row md:gap-10">
        <div className="shrink-0">
          <div className="size-24 sm:size-28 lg:size-40 bg-secondary/30 rounded-xl overflow-hidden flex items-center justify-center border border-border/50 shadow-sm">
            {!imageError && avatarSrc ? (
              <img
                src={avatarSrc}
                alt={profile.user.name}
                className="w-full h-full object-cover"
                onError={() => setImageError(true)}
              />
            ) : (
              <User className="w-16 h-16 text-muted-foreground/40" />
            )}
          </div>
        </div>

        <div className="min-w-0 w-full flex-1 space-y-3 sm:space-y-4 mt-1">
          <div className="min-w-0">
            <div className="flex min-w-0 items-start gap-2">
              <h1 className="min-w-0 text-2xl sm:text-3xl lg:text-4xl font-bold tracking-tight break-words wrap-anywhere">
                {profile.user.name}
              </h1>
              {isOwnProfile ? (
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  className="rounded-full"
                  onClick={openEditDialog}
                  title="Редактировать профиль"
                >
                  <Pencil className="size-4" />
                </Button>
              ) : null}
            </div>
          </div>

          <p className="max-w-3xl text-base sm:text-lg text-muted-foreground leading-relaxed whitespace-pre-wrap wrap-anywhere">
            {profile.user.bio}
          </p>
        </div>
      </div>

      <div className="border-t border-border/40 pt-8">
        <h2 className="text-2xl font-bold mb-6 hidden lg:block">
          Материалы автора
        </h2>
        <MaterialsSection materials={profile.materials.items} />
      </div>

      <Dialog
        open={isEditOpen}
        onOpenChange={(open) => {
          setIsEditOpen(open);
          if (!open) {
            setEditError("");
          }
        }}
      >
        <DialogContent className="sm:max-w-xl">
          <DialogHeader>
            <DialogTitle>Редактирование профиля</DialogTitle>
            <DialogDescription>
              Здесь можно изменить никнейм, аватар и описание профиля.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-5">
            <div className="space-y-2">
              <Label>Аватар</Label>
              <div className="flex items-center gap-4">
                <div className="size-16 overflow-hidden rounded-lg border border-border bg-muted/30 flex items-center justify-center">
                  {avatarDraft ? (
                    <img
                      src={avatarDraft}
                      alt="Превью аватара"
                      className="size-full object-cover"
                    />
                  ) : (
                    <User className="size-7 text-muted-foreground/60" />
                  )}
                </div>
                <input
                  ref={avatarInputRef}
                  type="file"
                  accept="image/*"
                  onChange={handleAvatarChange}
                  className="hidden"
                />
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => avatarInputRef.current?.click()}
                >
                  <Upload className="size-4" />
                  Загрузить аватар
                </Button>
              </div>
            </div>

            <div className="space-y-2">
              <Label htmlFor="profile-edit-username">Никнейм</Label>
              <Input
                id="profile-edit-username"
                value={nicknameDraft}
                onChange={(event) => setNicknameDraft(event.target.value)}
                maxLength={120}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="profile-edit-bio">О себе</Label>
              <Textarea
                id="profile-edit-bio"
                value={bioDraft}
                onChange={(event) => setBioDraft(event.target.value)}
                maxLength={MAX_BIO_LENGTH}
                className="min-h-28"
              />
              <p className="text-xs text-muted-foreground text-right">
                {bioDraft.length}/{MAX_BIO_LENGTH}
              </p>
            </div>

            {editError ? (
              <p className="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
                {editError}
              </p>
            ) : null}
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => setIsEditOpen(false)}
              disabled={isSavingProfile}
            >
              Отмена
            </Button>
            <Button
              type="button"
              onClick={handleSaveProfile}
              disabled={isSavingProfile}
            >
              {isSavingProfile ? (
                <>
                  <LoaderCircle className="size-4 animate-spin" />
                  Сохраняем...
                </>
              ) : (
                "Сохранить"
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ProfilePage;
