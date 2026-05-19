import { Pencil, User } from "lucide-react";

import { User as UserModel } from "@/entities/user/model";
import { Button } from "@/shared/ui";

interface Props {
  user: UserModel;
  imageError: boolean;
  avatarSrc: string | null;
  isOwnProfile: boolean;
  onImageError: () => void;
  onEditClick: () => void;
}

export const ProfileHeader = ({
  user,
  imageError,
  avatarSrc,
  isOwnProfile,
  onImageError,
  onEditClick,
}: Props) => (
  <div className="flex flex-col items-start gap-4 md:flex-row md:gap-10">
    <div className="shrink-0">
      <div className="size-24 sm:size-28 lg:size-40 bg-secondary/30 rounded-xl overflow-hidden flex items-center justify-center border border-border/50 shadow-sm">
        {!imageError && avatarSrc ? (
          <img
            src={avatarSrc}
            alt={user.name}
            className="w-full h-full object-cover"
            onError={onImageError}
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
            {user.name}
          </h1>
          {isOwnProfile && (
            <Button
              type="button"
              variant="ghost"
              size="icon-sm"
              className="rounded-full"
              onClick={onEditClick}
              title="Редактировать профиль"
            >
              <Pencil className="size-4" />
            </Button>
          )}
        </div>
      </div>
      <p className="max-w-3xl text-base sm:text-lg text-muted-foreground leading-relaxed whitespace-pre-wrap wrap-anywhere">
        {user.bio}
      </p>
    </div>
  </div>
);
