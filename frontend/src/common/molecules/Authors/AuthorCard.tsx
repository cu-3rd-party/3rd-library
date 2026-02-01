import { User } from "lucide-react";

import { Card } from "@/components/ui/card";

export type Author = {
  id: number;
  name: string;
  avatar?: string;
  isFollowing?: boolean;
};

type AuthorCardProps = {
  author: Author;
  onClick: (id: number) => void;
};

export const AuthorCard = ({ author, onClick }: AuthorCardProps) => {
  return (
    <Card
      onClick={() => onClick(author.id)}
      className="aspect-square flex flex-col justify-between p-3 md:p-5 border-border/50 bg-card hover:border-orange-500/30 transition-all duration-300 group cursor-pointer"
    >
      {/* Аватар / Иконка */}
      <div className="w-full flex-1 flex items-start justify-start mb-2 md:mb-4 min-h-0">
        <div className="h-full aspect-square rounded-xl bg-secondary/50 flex items-center justify-center overflow-hidden border border-transparent group-hover:border-border/30 transition-colors">
          {author.avatar ? (
            <img
              src={author.avatar}
              alt={author.name}
              className="w-full h-full object-cover"
            />
          ) : (
            <User className="h-1/2 w-1/2 text-muted-foreground/40" />
          )}
        </div>
      </div>

      {/* Имя автора */}
      <div className="flex items-center w-full mt-auto">
        <span className="font-bold text-xs sm:text-sm md:text-base leading-tight line-clamp-2 text-left w-full group-hover:text-orange-600 transition-colors">
          {author.name}
        </span>
      </div>
    </Card>
  );
};
