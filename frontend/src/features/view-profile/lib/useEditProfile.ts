import { ChangeEvent, Dispatch, SetStateAction, useRef, useState } from "react";
import { useNavigate } from "react-router-dom";

import {
  persistCurrentAuthUser,
  setAccessToken,
} from "@/entities/session/lib";
import { StoredAuthUser } from "@/entities/session/model";
import { updateCurrentUser } from "@/features/edit-profile/api";
import { AUTHORS_PREFIX } from "@/shared/constants";

import { UserProfileResponse } from "../model";
import { MAX_BIO_LENGTH, readFileAsDataUrl, resolvePfpUrl } from "./helpers";

interface Options {
  profile: UserProfileResponse | null;
  setProfile: Dispatch<SetStateAction<UserProfileResponse | null>>;
  currentAuthUser: StoredAuthUser | null;
  setCurrentAuthUser: Dispatch<SetStateAction<StoredAuthUser | null>>;
  setImageError: (value: boolean) => void;
}

export const useEditProfile = ({
  profile,
  setProfile,
  currentAuthUser,
  setCurrentAuthUser,
  setImageError,
}: Options) => {
  const navigate = useNavigate();

  const [isEditOpen, setIsEditOpen] = useState(false);
  const [nicknameDraft, setNicknameDraft] = useState("");
  const [bioDraft, setBioDraft] = useState("");
  const [avatarDraft, setAvatarDraft] = useState<string | null>(null);
  const [editError, setEditError] = useState("");
  const [isSavingProfile, setIsSavingProfile] = useState(false);
  const avatarInputRef = useRef<HTMLInputElement>(null);

  const openEditDialog = () => {
    if (!profile) return;
    setNicknameDraft(profile.user.name);
    setBioDraft(profile.user.bio);
    setAvatarDraft(resolvePfpUrl(profile.user.image));
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
      .then((dataUrl) => { setAvatarDraft(dataUrl); setEditError(""); })
      .catch((err: unknown) => {
        console.error(err);
        setEditError("Не удалось прочитать изображение. Попробуйте другой файл.");
      });
  };

  const handleSaveProfile = async () => {
    if (!currentAuthUser) return;
    if (!currentAuthUser.email.trim()) {
      setEditError("Не удалось определить email текущего пользователя. Выполните вход заново.");
      return;
    }
    const nextUsername = nicknameDraft.trim();
    if (!nextUsername) { setEditError("Никнейм не может быть пустым."); return; }
    const nextBio = bioDraft.trim();
    if (nextBio.length > MAX_BIO_LENGTH) {
      setEditError(`Bio не должен превышать ${MAX_BIO_LENGTH} символов.`);
      return;
    }

    setIsSavingProfile(true);
    setEditError("");
    try {
      const response = await updateCurrentUser({
        email: currentAuthUser.email,
        username: nextUsername,
        bio: nextBio,
        image: profile?.user.image || null,
      });
      const nextAuthUser: StoredAuthUser = {
        email: response.user.email,
        username: response.user.username,
        bio: response.user.bio || "",
        image: response.user.image,
        roles: response.user.roles || currentAuthUser.roles || ["user"],
      };
      setAccessToken(`Token ${response.user.token}`);
      persistCurrentAuthUser(nextAuthUser);
      setCurrentAuthUser(nextAuthUser);
      setProfile((current) =>
        current
          ? { ...current, user: { ...current.user, id: nextAuthUser.username, name: nextAuthUser.username, bio: nextAuthUser.bio, image: nextAuthUser.image } }
          : current,
      );
      setImageError(false);
      setIsEditOpen(false);
      navigate(`${AUTHORS_PREFIX}/${encodeURIComponent(nextAuthUser.username)}`, { replace: true });
    } catch (error) {
      console.error(error);
      setEditError(error instanceof Error ? error.message : "Не удалось сохранить изменения профиля.");
    } finally {
      setIsSavingProfile(false);
    }
  };

  return {
    isEditOpen,
    setIsEditOpen,
    nicknameDraft,
    setNicknameDraft,
    bioDraft,
    setBioDraft,
    avatarDraft,
    avatarInputRef,
    editError,
    isSavingProfile,
    openEditDialog,
    handleAvatarChange,
    handleSaveProfile,
  };
};
