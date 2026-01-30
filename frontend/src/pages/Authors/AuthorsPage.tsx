import { useState } from "react";
import { Search, Plus, Check, User } from "lucide-react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";

type Author = {
  id: number;
  name: string;
  avatar?: string;
  isFollowing?: boolean;
};

const MOCK_AUTHORS: Author[] = [
  { id: 1, name: "Матан Матанович" },
  { id: 2, name: "Линал Линалович" },
  { id: 3, name: "Это Дима Трушин" },
  { id: 4, name: "Это магадан" },
  { id: 5, name: "Горо горо)" },
  { id: 6, name: "Матан Матанович" },
  { id: 7, name: "Матан Матанович" },
  { id: 8, name: "Матан Матанович" },
];

export const AuthorsPage = () => {
  const [searchQuery, setSearchQuery] = useState("");
  const [followingIds, setFollowingIds] = useState<number[]>([]);

  const toggleFollow = (id: number) => {
    setFollowingIds((prev) =>
      prev.includes(id) ? prev.filter((i) => i !== id) : [...prev, id],
    );
  };

  const filteredAuthors = MOCK_AUTHORS.filter((author) =>
    author.name.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  return (
    // 1. Изменили контейнер: на мобильных w-full + px-4, на десктопе w-5/6
    <div className="container w-full md:w-5/6 mx-auto max-w-screen-2xl py-4 md:py-6 px-4 md:px-0 space-y-6 md:space-y-8">

      {/* Поиск */}
      <div className="flex justify-end">
        <div className="relative w-full md:w-72">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            // Увеличил высоту на мобильных для удобства нажатия
            className="pl-9 bg-secondary/30 border-border/50 focus-visible:ring-orange-500 rounded-lg h-10 md:h-10"
          />
        </div>
      </div>

      {/* Сетка авторов */}
      {/* 2. Изменили gap: gap-3 на мобильных, gap-6 на ПК */}
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3 md:gap-6">
        {filteredAuthors.length > 0 ? (
          filteredAuthors.map((author) => {
            const isFollowed = followingIds.includes(author.id);

            return (
              <Card
                key={author.id}
                // 3. Уменьшили padding: p-3 на мобильных, p-5 на ПК
                className="aspect-square flex flex-col justify-between p-3 md:p-5 border-border/50 bg-card hover:border-orange-500/30 transition-all duration-300 group"
              >
                {/* Блок с аватаром */}
                {/* mb-2 на мобильных, mb-4 на ПК */}
                <div className="w-full flex-1 flex items-start justify-start mb-2 md:mb-4 min-h-0">
                  <div className="h-full aspect-square rounded-xl bg-secondary/50 flex items-center justify-center overflow-hidden">
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

                {/* Нижняя часть: Имя и Кнопка */}
                <div className="flex items-center justify-between w-full mt-auto gap-2">
                  {/* 4. Адаптивный размер текста: text-xs на мобильных, text-base на ПК */}
                  <span className="font-bold text-xs sm:text-sm md:text-base leading-tight line-clamp-2 text-left">
                    {author.name}
                  </span>

                  <Button
                    size="icon"
                    className={`shrink-0 rounded-full w-8 h-8 md:w-10 md:h-10 transition-colors ${
                      isFollowed
                        ? "bg-green-500 hover:bg-green-600 text-white"
                        : "bg-orange-500 hover:bg-orange-600 text-white"
                    }`}
                    onClick={(e) => {
                      e.stopPropagation();
                      toggleFollow(author.id);
                    }}
                  >
                    {isFollowed ? (
                      <Check className="h-3 w-3 md:h-5 md:w-5" />
                    ) : (
                      <Plus className="h-3 w-3 md:h-5 md:w-5" />
                    )}
                  </Button>
                </div>
              </Card>
            );
          })
        ) : (
          <div className="col-span-full text-center py-20 text-muted-foreground">
            Авторы не найдены
          </div>
        )}
      </div>
    </div>
  );
};