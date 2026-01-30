import { useState } from "react";
import { useParams } from "react-router-dom";
import { Bookmark, FileDown, FileText } from "lucide-react";

import { Button } from "@/components/ui/button";

// Моковые данные для примера (в реальности ты будешь делать запрос по id)
const MOCK_MATERIAL = {
  id: "1",
  title: "Линейная алгебра",
  description:
    "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
  fileName: "lecture_notes.pdf",
  fileSize: "2.4 MB",
};

export const MaterialDetailPage = () => {
  const { id } = useParams();

  const [isBookmarked, setIsBookmarked] = useState(false);

  const handleDownload = () => {
    alert(`Скачивание файла: ${MOCK_MATERIAL.fileName}`);
  };

  return (
    <div className="container w-5/6 m-auto max-w-screen-2xl py-10 space-y-6">
      <div className="flex justify-between items-start gap-4">
        <h1 className="text-3xl md:text-4xl font-bold text-foreground leading-tight">
          {MOCK_MATERIAL.title}
        </h1>

        <Button
          variant="ghost"
          size="icon-lg"
          onClick={() => setIsBookmarked(!isBookmarked)}
          className="shrink-1  hover:bg-transparent h-12 w-12"
        >
          <Bookmark
            className={`h-22 w-22 transition-all duration-300 ${
              isBookmarked
                ? "fill-orange-500 text-orange-500" // Активная закладка
                : "text-muted-foreground hover:text-orange-500" // Неактивная
            }`}
          />
        </Button>
      </div>

      <div className="text-muted-foreground text-lg leading-relaxed max-w-4xl">
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
