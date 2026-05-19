import { LoaderCircle, Upload, User } from "lucide-react";
import { ChangeEvent, RefObject } from "react";

import { MAX_BIO_LENGTH } from "../lib/helpers";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  Textarea,
} from "@/shared/ui";

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  nicknameDraft: string;
  onNicknameChange: (value: string) => void;
  bioDraft: string;
  onBioChange: (value: string) => void;
  avatarDraft: string | null;
  avatarInputRef: RefObject<HTMLInputElement | null>;
  editError: string;
  isSaving: boolean;
  onAvatarChange: (event: ChangeEvent<HTMLInputElement>) => void;
  onSave: () => void;
}

export const EditProfileDialog = ({
  open,
  onOpenChange,
  nicknameDraft,
  onNicknameChange,
  bioDraft,
  onBioChange,
  avatarDraft,
  avatarInputRef,
  editError,
  isSaving,
  onAvatarChange,
  onSave,
}: Props) => (
  <Dialog
    open={open}
    onOpenChange={(isOpen) => {
      onOpenChange(isOpen);
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
                <img src={avatarDraft} alt="Превью аватара" className="size-full object-cover" />
              ) : (
                <User className="size-7 text-muted-foreground/60" />
              )}
            </div>
            <input
              ref={avatarInputRef}
              type="file"
              accept="image/*"
              onChange={onAvatarChange}
              className="hidden"
            />
            <div className="space-y-1">
              <Button
                type="button"
                variant="outline"
                disabled
                onClick={() => avatarInputRef.current?.click()}
              >
                <Upload className="size-4" />
                Загрузить аватар
              </Button>
              <p className="text-xs text-muted-foreground">Загрузка аватара пока недоступна</p>
            </div>
          </div>
        </div>

        <div className="space-y-2">
          <Label htmlFor="profile-edit-username">Никнейм</Label>
          <Input
            id="profile-edit-username"
            value={nicknameDraft}
            onChange={(e) => onNicknameChange(e.target.value)}
            maxLength={120}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="profile-edit-bio">О себе</Label>
          <Textarea
            id="profile-edit-bio"
            value={bioDraft}
            onChange={(e) => onBioChange(e.target.value)}
            maxLength={MAX_BIO_LENGTH}
            className="min-h-28"
          />
          <p className="text-xs text-muted-foreground text-right">
            {bioDraft.length}/{MAX_BIO_LENGTH}
          </p>
        </div>

        {editError && (
          <p className="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
            {editError}
          </p>
        )}
      </div>

      <DialogFooter>
        <Button type="button" variant="outline" onClick={() => onOpenChange(false)} disabled={isSaving}>
          Отмена
        </Button>
        <Button type="button" onClick={onSave} disabled={isSaving}>
          {isSaving ? (
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
);
