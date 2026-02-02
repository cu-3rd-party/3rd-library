import { ShieldCheck, User } from "lucide-react";
import { useState } from "react";

import { MaterialsSection } from "@/common/organisms";
import { Badge } from "@/components/ui/badge";
import { MOCK_USER } from "@/mocks";

const ProfilePage = () => {
  const [imageError, setImageError] = useState(false);
  
  return (
    <div className="container mx-auto max-w-screen-xl px-4 md:px-8 py-10 space-y-10">
      <div className="flex flex-col md:flex-row items-start gap-6 md:gap-10">
        <div className="shrink-0">
          <div className="w-32 h-32 md:w-40 md:h-40 bg-secondary/30 rounded-xl overflow-hidden flex items-center justify-center border border-border/50 shadow-sm">
          {!imageError ? (
            <img
              src={`/avatars/${MOCK_USER.id}.png`}
              alt={MOCK_USER.name}
              className="w-full h-full object-cover"
              onError={() => setImageError(true)}
            />
          ) : (
            <User className="w-16 h-16 text-muted-foreground/40" />
          )}
          </div>
        </div>

        <div className="space-y-4 mt-1">
          <div className="space-y-2">
            <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
              {MOCK_USER.name}
            </h1>

            {MOCK_USER.verified && (
              <Badge
                variant="secondary"
                className="bg-emerald-100 text-emerald-700 hover:bg-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400 border-emerald-200 dark:border-emerald-800 px-3 py-1 text-sm font-medium gap-1.5 w-fit flex items-center"
              >
                <ShieldCheck className="w-4 h-4" />
                Проверенный автор
              </Badge>
            )}
          </div>

          <p className="text-lg text-muted-foreground leading-relaxed max-w-2xl">
            {MOCK_USER.bio}
          </p>
        </div>
      </div>

      <div className="border-t border-border/40 pt-8">
        <h2 className="text-2xl font-bold mb-6 hidden md:block">
          Материалы автора
        </h2>
        { MOCK_USER.materials && (
          <MaterialsSection materials={MOCK_USER.materials} />
        )}
      </div>
    </div>
  );
};

export default ProfilePage;
