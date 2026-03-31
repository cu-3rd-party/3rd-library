import { User as UserIcon } from "lucide-react";
import { useState } from "react";

import { Card } from "@/components/ui/card";
import { MOCK_USER } from "@/mocks";
import { User } from "@/models/user";

type AuthorCardProps = {
  author: User;
  onClick: (id: string) => void;
};

export const AuthorCard = ({ author, onClick }: AuthorCardProps) => {
  const [imageError, setImageError] = useState(false);
  return (
    <Card
      onClick={() => onClick(author.id)}
      className="aspect-square flex flex-col justify-between p-3 lg:p-5 bg-card border-border
               hover:border-ring transition-all duration-(--std-duration) group cursor-pointer border"
    >
      <div className="w-full flex-1 flex items-start justify-start mb-2 lg:mb-4 min-h-0">
        <div className="h-full aspect-square rounded-xl flex items-center justify-center overflow-hidden">
          {!imageError ? (
            <img
              src={`/avatars/${MOCK_USER.id}.png`}
              alt={author.name}
              className="w-full h-full object-cover"
              onError={() => setImageError(true)}
            />
          ) : (
            <UserIcon className="h-3/4 w-3/4 " />
          )}
        </div>
      </div>

      <div className="flex items-center w-full mt-auto">
        <span className="font-bold text-xs sm:text-sm lg:text-base leading-tight line-clamp-2 text-left w-full">
          {author.name}
        </span>
      </div>
    </Card>
  );
};
