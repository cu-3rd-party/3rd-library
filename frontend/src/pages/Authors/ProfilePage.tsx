import { useParams } from "react-router-dom";

import {
  EditProfileDialog,
  ProfileHeader,
  useEditProfile,
  useProfile,
} from "@/features/view-profile";
import { MaterialsSection } from "@/widgets/materials-section/ui";

const ProfilePage = () => {
  const { id } = useParams();

  const {
    profile,
    setProfile,
    isLoading,
    isError,
    imageError,
    setImageError,
    isOwnProfile,
    avatarSrc,
    currentAuthUser,
    setCurrentAuthUser,
  } = useProfile(id);

  const editDialog = useEditProfile({
    profile,
    setProfile,
    currentAuthUser,
    setCurrentAuthUser,
    setImageError,
  });

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
    <div className="container mx-auto max-w-screen-xl overflow-x-hidden px-4 py-8 lg:px-8 lg:py-10 space-y-8 lg:space-y-10">
      <ProfileHeader
        user={profile.user}
        imageError={imageError}
        avatarSrc={avatarSrc}
        isOwnProfile={isOwnProfile}
        onImageError={() => setImageError(true)}
        onEditClick={editDialog.openEditDialog}
      />

      <div className="border-t border-border/40 pt-8">
        <h2 className="text-2xl font-bold mb-6 hidden lg:block">Материалы автора</h2>
        <MaterialsSection materials={profile.materials.items} />
      </div>

      <EditProfileDialog
        open={editDialog.isEditOpen}
        onOpenChange={editDialog.setIsEditOpen}
        nicknameDraft={editDialog.nicknameDraft}
        onNicknameChange={editDialog.setNicknameDraft}
        bioDraft={editDialog.bioDraft}
        onBioChange={editDialog.setBioDraft}
        avatarDraft={editDialog.avatarDraft}
        avatarInputRef={editDialog.avatarInputRef}
        editError={editDialog.editError}
        isSaving={editDialog.isSavingProfile}
        onAvatarChange={editDialog.handleAvatarChange}
        onSave={editDialog.handleSaveProfile}
      />
    </div>
  );
};

export default ProfilePage;
