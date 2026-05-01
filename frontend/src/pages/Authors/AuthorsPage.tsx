import { Search } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

import { AuthorsGrid } from "@/common/organisms";
import { Input } from "@/components/ui/input";
import { fetchJson, resolveApiUrl } from "@/lib/api";
import { MOCK_AUTHORS } from "@/mocks/mockData";
import { User } from "@/models/user";

type UsersResponse = {
  items: User[];
  page: number;
  limit: number;
  total: number;
};

const AuthorsPage = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");
  const [authors, setAuthors] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
    const abortController = new AbortController();

    const fetchUsers = async () => {
      setIsLoading(true);
      setIsError(false);

      try {
        const payload = await fetchJson<UsersResponse>(
          resolveApiUrl("/users"),
          {
            signal: abortController.signal,
          },
        );
        setAuthors(payload.items);
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          console.warn("[Users] Falling back to local mock list.");
          setAuthors(MOCK_AUTHORS);
          setIsError(false);
          return;
        }

        console.error(error);
        setIsError(true);
      } finally {
        if (!abortController.signal.aborted) {
          setIsLoading(false);
        }
      }
    };

    fetchUsers();

    return () => abortController.abort();
  }, []);

  const filteredAuthors = useMemo(
    () =>
      authors.filter((author) =>
        author.name.toLowerCase().includes(searchQuery.toLowerCase()),
      ),
    [authors, searchQuery],
  );

  const handleAuthorClick = (id: string) => {
    navigate(`/authors/${id}`);
  };

  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl space-y-6 lg:space-y-8">
      <div className="flex justify-end">
        <div className="relative w-full lg:w-72">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9 border-border rounded-lg h-10 lg:h-10"
          />
        </div>
      </div>

      {isLoading ? (
        <div className="col-span-full text-center py-20 text-muted-foreground bg-secondary/10 rounded-xl border border-dashed border-border/50">
          Загружаем пользователей...
        </div>
      ) : isError ? (
        <div className="col-span-full text-center py-20 text-destructive bg-secondary/10 rounded-xl border border-dashed border-border/50">
          Не удалось загрузить пользователей
        </div>
      ) : (
        <AuthorsGrid
          authors={filteredAuthors}
          onAuthorClick={handleAuthorClick}
        />
      )}
    </div>
  );
};

export default AuthorsPage;
