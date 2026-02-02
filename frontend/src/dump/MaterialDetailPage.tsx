import { Bookmark, FileDown, FileText } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

const DIFFICULTY_CONFIG: Record<string, { label: string; className: string }> =
  {
    none: {
      label: "Без уровня",
      className:
        "bg-secondary text-secondary-foreground hover:bg-secondary/80 border-transparent",
    },
    blue: {
      label: "Синий уровень",
      className:
        "bg-blue-500 text-white hover:bg-blue-600 border-transparent shadow-md shadow-blue-500/20",
    },
    red: {
      label: "Красный уровень",
      className:
        "bg-red-500 dark:bg-red-500 dark:text-white hover:bg-red-600 border-transparent shadow-md shadow-red-500/20",
    },
    black: {
      label: "Черный уровень",
      className:
        "bg-black text-white hover:bg-neutral-900 border-transparent dark:border-white/30 dark:border shadow-md shadow-black/20",
    },
  };

const MOCK_MATERIAL = {
  id: "1",
  title: "Линейная алгебра",
  description:
    "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
  fileName: "lecture_notes.pdf",
  fileSize: "2.4 MB",
  difficulty: "red",
};

export const MaterialDetailPage = () => {
  const { id } = useParams();
  const [isBookmarked, setIsBookmarked] = useState(false);

  const handleDownload = () => {
    alert(`Скачивание файла: ${MOCK_MATERIAL.fileName}`);
  };

  // Получаем настройки для текущей сложности
  const difficultyData =
    DIFFICULTY_CONFIG[MOCK_MATERIAL.difficulty] || DIFFICULTY_CONFIG.none;

  return (
    <div className="container w-5/6 m-auto max-w-screen-2xl py-10 space-y-6">
      {/* Верхняя секция: Заголовок + Кнопка закладки */}
      <div className="flex justify-between items-start gap-4">
        <div className="space-y-4">
          <h1 className="text-3xl md:text-4xl font-bold text-foreground leading-tight">
            {MOCK_MATERIAL.title}
          </h1>

          {/* 3. ОТОБРАЖЕНИЕ УРОВНЯ СЛОЖНОСТИ */}
          {/* Показываем бейдж, только если уровень не 'none' (или показываем всегда, если так задумано) */}
          <div className="flex items-center gap-2">
            <Badge
              className={cn(
                "text-sm px-3 py-1 pointer-events-none",
                difficultyData.className,
              )}
            >
              {difficultyData.label}
            </Badge>
          </div>
        </div>

        <Button
          variant="ghost"
          size="icon"
          onClick={() => setIsBookmarked(!isBookmarked)}
          className="shrink-0 hover:bg-transparent h-12 w-12 rounded-full"
        >
          <Bookmark
            className={`h-8 w-8 transition-all duration-300 ${
              isBookmarked
                ? "fill-orange-500 text-orange-500"
                : "text-muted-foreground hover:text-orange-500"
            }`}
          />
        </Button>
      </div>

      <div className="text-muted-foreground text-lg leading-relaxed max-w-4xl border-t border-border/40 pt-6">
        {MOCK_MATERIAL.description}
      </div>

      <div
        onClick={handleDownload}
        className="mt-8 w-full h-64 bg-card border-2 border-dashed border-border/50 rounded-xl flex flex-col items-center justify-center cursor-pointer hover:bg-secondary/50 hover:border-orange-500/50 transition-all group"
      >
        <div className="flex flex-col items-center gap-4 text-muted-foreground group-hover:text-foreground transition-colors">
          <div className="p-4 bg-background rounded-full shadow-sm">
            <FileText className="h-10 w-10 text-orange-500" />
          </div>
          <div className="text-center">
            <p className="text-xl font-semibold">Скачать PDF</p>
            <p className="text-sm opacity-70">
              {MOCK_MATERIAL.fileName} • {MOCK_MATERIAL.fileSize}
            </p>
          </div>
          <Button variant="outline" className="mt-2 gap-2 pointer-events-none">
            <FileDown className="h-4 w-4" />
            Загрузить
          </Button>
        </div>
      </div>
    </div>
  );
};
