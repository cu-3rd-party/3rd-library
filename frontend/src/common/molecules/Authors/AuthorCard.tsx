import { User as UserIcon } from "lucide-react";
import { useEffect, useState } from "react";

import { Card } from "@/components/ui/card";
import { User } from "@/models/user";

type AuthorCardProps = {
  author: User;
  onClick: (id: string) => void;
};

const UUID_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

export const AuthorCard = ({ author, onClick }: AuthorCardProps) => {
  const defaultAvatarPath = UUID_PATTERN.test(author.id)
    ? `/avatars/${encodeURIComponent(author.id)}.png`
    : null;

  const [avatarSrc, setAvatarSrc] = useState<string | null>(
    author.image || defaultAvatarPath,
  );

  useEffect(() => {
    setAvatarSrc(author.image || defaultAvatarPath);
  }, [author.image, defaultAvatarPath]);

  const handleImageError = () => {
    if (defaultAvatarPath && avatarSrc !== defaultAvatarPath) {
      setAvatarSrc(defaultAvatarPath);
      return;
    }

    setAvatarSrc(null);
  };

  return (
    <Card
      onClick={() => onClick(author.id)}
      className="aspect-square flex flex-col justify-between p-3 lg:p-5 bg-card border-border
               hover:border-ring transition-all duration-(--std-duration) group cursor-pointer border"
    >
      <div className="w-full flex-1 flex items-start justify-start mb-2 lg:mb-4 min-h-0">
        <div className="h-full aspect-square rounded-xl flex items-center justify-center overflow-hidden">
          {avatarSrc ? (
            <img
              src={avatarSrc}
              alt={author.name}
              className="w-full h-full object-cover"
              onError={handleImageError}
            />
          ) : (
            <UserIcon className="h-3/4 w-3/4 text-muted-foreground/70" />
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
