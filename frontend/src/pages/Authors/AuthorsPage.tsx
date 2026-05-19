import { Search } from "lucide-react";

import { useAuthors } from "@/features/browse-authors";
import { Input } from "@/shared/ui";
import { AuthorsGrid } from "@/widgets/authors-grid/ui";

const AuthorsPage = () => {
  const {
    searchQuery,
    setSearchQuery,
    filteredAuthors,
    isLoading,
    isError,
    handleAuthorClick,
  } = useAuthors();

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
        <AuthorsGrid authors={filteredAuthors} onAuthorClick={handleAuthorClick} />
      )}
    </div>
  );
};

export default AuthorsPage;
