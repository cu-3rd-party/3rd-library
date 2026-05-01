import { ShieldCheck, User } from "lucide-react";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { MaterialsSection } from "@/common/organisms";
import { Badge } from "@/components/ui/badge";
import { fetchJson, resolveApiUrl } from "@/lib/api";
import { MOCK_AUTHORS, MOCK_SUBMISSIONS, MOCK_USER } from "@/mocks/mockData";
import { Material, User as UserModel } from "@/models";

type UserWithMaterialsResponse = {
  user: UserModel;
  materials: {
    items: Material[];
    page: number;
    limit: number;
    total: number;
  };
};

const getFallbackProfile = (
  userId: string,
): UserWithMaterialsResponse | null => {
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
  const { id } = useParams();
  const [imageError, setImageError] = useState(false);
  const [profile, setProfile] = useState<UserWithMaterialsResponse | null>(
    null,
  );
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
    const userId = id || "";
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
        const payload = await fetchJson<UserWithMaterialsResponse>(
          resolveApiUrl(`/users/${userId}`),
          { signal: abortController.signal },
        );
        setProfile(payload);
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
  }, [id]);

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

  return (
    <div className="container mx-auto max-w-screen-xl px-4 lg:px-8 py-10 space-y-10">
      <div className="flex flex-col lg:flex-row items-start gap-6 lg:gap-10">
        <div className="shrink-0">
          <div className="w-32 h-32 lg:w-40 lg:h-40 bg-secondary/30 rounded-xl overflow-hidden flex items-center justify-center border border-border/50 shadow-sm">
            {!imageError ? (
              <img
                src={`/avatars/${profile.user.id}.png`}
                alt={profile.user.name}
                className="w-full h-full object-cover"
                onError={() => setImageError(true)}
              />
            ) : (
              <User className="w-16 h-16 text-muted-foreground/40" />
            )}
          </div>
        </div>

        <div className="space-y-4 mt-1">
          <div className="space-y-2">
            <h1 className="text-3xl lg:text-4xl font-bold tracking-tight">
              {profile.user.name}
            </h1>

            {profile.user.verified && (
              <Badge
                variant="secondary"
                className="bg-emerald-100 text-emerald-700 hover:bg-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400 border-emerald-200 dark:border-emerald-800 px-3 py-1 text-sm font-medium gap-1.5 w-fit flex items-center"
              >
                <ShieldCheck className="w-4 h-4" />
                Проверенный автор
              </Badge>
            )}
          </div>

          <p className="text-lg text-muted-foreground leading-relaxed max-w-2xl">
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
    </div>
  );
};

export default ProfilePage;
