import { Author, AuthorCard} from "@/common/molecules";

type AuthorsGridProps = {
  authors: Author[];
  onAuthorClick: (id: number) => void;
};

export const AuthorsGrid = ({ authors, onAuthorClick }: AuthorsGridProps) => {
  if (authors.length === 0) {
    return (
      <div className="col-span-full text-center py-20 text-muted-foreground bg-secondary/10 rounded-xl border border-dashed border-border/50">
        Авторы не найдены
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3 md:gap-6">
      {authors.map((author) => (
        <AuthorCard
          key={author.id}
          author={author}
          onClick={onAuthorClick}
        />
      ))}
    </div>
  );
};