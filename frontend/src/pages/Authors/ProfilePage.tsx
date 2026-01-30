import { useState } from "react";
import { Search, ShieldCheck, User } from "lucide-react";

import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";


type UserProfile = {
  name: string;
  bio: string;
  avatar?: string;
  isVerified: boolean;
};

type Material = {
  id: number;
  title: string;
  date: string;
  previewText: string;
  tags: string[];
  author: string;
};

const MOCK_PROFILE: UserProfile = {
  name: "Даниил Матанович",
  bio: "Облизал весь матан с ног до головы",
  avatar: "/pwa-144x144.png",
  isVerified: true,
};

const AUTHOR_MATERIALS: Material[] = [
  {
    id: 1,
    title: "Полный сборник лонгридов по матану",
    date: "29.01.2026",
    previewText:
      "9. НЕВЫРОЖДЕННЫЕ МАТРИЦЫ. Нет, не начнем. Невырожденная матрица — это квадратная матрица, у которой определитель не равен нулю. Остальные фокусы с матрицами здесь рассматриваться не будут...",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
  },
  {
    id: 2,
    title: "Введение в Линейную Алгебру",
    date: "28.01.2026",
    previewText:
      "Линейное пространство, базис, размерность. В этом лонгриде мы разберем основные понятия, которые потребуются для решения задач...",
    tags: ["Линал", "1 курс"],
    author: "Даниил Матанович",
  },
  {
    id: 3,
    title: "Пределы последовательностей",
    date: "30.01.2026",
    previewText:
      "Определение предела по Коши. Epsilon-N определение. Примеры вычисления простейших пределов.",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
  },
];

export const ProfilePage = () => {
  const [searchQuery, setSearchQuery] = useState("");

  const filteredMaterials = AUTHOR_MATERIALS.filter((item) =>
    item.title.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="container mx-auto max-w-screen-xl px-4 md:px-8 py-10 space-y-10">

      <div className="flex flex-col md:flex-row items-start gap-6 md:gap-10">

        <div className="shrink-0">
          <div className="w-32 h-32 md:w-40 md:h-40 bg-secondary/30 rounded-xl overflow-hidden flex items-center justify-center border border-border/50">
            {MOCK_PROFILE.avatar ? (
              <img
                src={MOCK_PROFILE.avatar}
                alt={MOCK_PROFILE.name}
                className="w-full h-full object-cover"
              />
            ) : (
              <User className="w-16 h-16 text-muted-foreground/40" />
            )}
          </div>
        </div>

        <div className="space-y-3 mt-1">
          <h1 className="text-3xl md:text-4xl font-bold">{MOCK_PROFILE.name}</h1>
          <p className="text-lg text-muted-foreground leading-relaxed">
            {MOCK_PROFILE.bio}
          </p>

          {MOCK_PROFILE.isVerified && (
            <Badge
              variant="secondary"
              className="bg-emerald-100 text-emerald-700 hover:bg-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400 border-emerald-200 dark:border-emerald-800 px-3 py-1 text-sm font-medium gap-1.5"
            >
              <ShieldCheck className="w-4 h-4" />
              Проверенный автор
            </Badge>
          )}
        </div>
      </div>

      <div className="flex justify-end border-t border-border/40 pt-8">
        <div className="relative w-full md:w-80">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9 bg-secondary/30 border-border/50 focus-visible:ring-orange-500 rounded-lg h-10 md:h-12 text-base"
          />
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        {filteredMaterials.length > 0 ? (
          filteredMaterials.map((material) => (
            <Card
              key={material.id}
              className="flex flex-col h-full border-border/50 bg-card hover:border-orange-500/50 transition-colors cursor-pointer group"
            >
              <CardHeader className="pb-2 h-[110px]">
                <CardTitle className="text-xl font-bold leading-tight group-hover:text-orange-500 transition-colors line-clamp-2">
                  {material.title}
                </CardTitle>
                <p className="text-xs text-muted-foreground mt-1">
                  {material.date}
                </p>
              </CardHeader>

              <CardContent className="flex-grow pt-0">
                <p className="text-sm text-muted-foreground line-clamp-6 leading-relaxed">
                  {material.previewText}
                </p>
              </CardContent>

              <CardFooter className="flex items-center justify-between pt-4">
                <div className="flex gap-2">
                  {material.tags.map((tag, idx) => (
                    <Badge
                      key={idx}
                      variant="secondary"
                      className={`rounded-md px-2 py-0.5 text-xs font-normal ${
                        idx === 0
                          ? "bg-red-100/10 text-red-400 hover:bg-red-100/20"
                          : "bg-secondary text-secondary-foreground"
                      }`}
                    >
                      {tag}
                    </Badge>
                  ))}
                </div>
                <span className="text-sm font-medium text-foreground">
                  {material.author}
                </span>
              </CardFooter>
            </Card>
          ))
        ) : (
          <div className="col-span-full text-center py-20 text-muted-foreground">
            Материалы не найдены
          </div>
        )}
      </div>
    </div>
  );
};