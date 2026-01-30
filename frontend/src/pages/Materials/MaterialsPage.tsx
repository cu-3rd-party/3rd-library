import { useState } from "react";
import { Search, Filter } from "lucide-react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

type Material = {
  id: number;
  course: 1 | 2;
  title: string;
  date: string;
  previewText: string;
  tags: string[];
  author: string;
};

const ALL_SUBJECTS = ["Матан", "Линал", "Диффуры"];

const MOCK_MATERIALS: Material[] = [
  {
    id: 1,
    course: 1,
    title: "Полный сборник лонгридов по матану",
    date: "29.01.2026",
    previewText:
      "9. НЕВЫРОЖДЕННЫЕ МАТРИЦЫ. Нет, не начнем. Невырожденная матрица — это квадратная матрица...",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
  },
  {
    id: 2,
    course: 1,
    title: "Введение в Линейную Алгебру",
    date: "28.01.2026",
    previewText:
      "Линейное пространство, базис, размерность. В этом лонгриде мы разберем основные понятия...",
    tags: ["Линал", "1 курс"],
    author: "Алексей Векторов",
  },
  {
    id: 3,
    course: 2,
    title: "Дифференциальные уравнения: основы",
    date: "15.01.2026",
    previewText:
      "Диффуры — это не страшно. Разделяем переменные, интегрируем обе части и получаем общее решение...",
    tags: ["Диффуры", "2 курс"],
    author: "Мария Интегралова",
  },
  {
    id: 4,
    course: 1,
    title: "Пределы последовательностей",
    date: "30.01.2026",
    previewText:
      "Определение предела по Коши. Epsilon-N определение. Примеры вычисления...",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
  },
];

export const MaterialsPage = () => {
  const [selectedCourse, setSelectedCourse] = useState<1 | 2>(1);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedSubjects, setSelectedSubjects] = useState<string[]>([]);

  const toggleSubject = (subject: string) => {
    setSelectedSubjects((prev) =>
      prev.includes(subject)
        ? prev.filter((s) => s !== subject)
        : [...prev, subject],
    );
  };

  const filteredMaterials = MOCK_MATERIALS.filter((item) => {
    const matchCourse = item.course === selectedCourse;

    const matchSearch =
      item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.previewText.toLowerCase().includes(searchQuery.toLowerCase());

    const matchSubject =
      selectedSubjects.length === 0 ||
      item.tags.some((tag) => selectedSubjects.includes(tag));

    return matchCourse && matchSearch && matchSubject;
  });

  return (
    <div className="container w-5/6 m-auto max-w-screen-2xl py-6 space-y-8">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div className="flex gap-2">
          <Button
            onClick={() => setSelectedCourse(1)}
            className={`font-semibold text-sm px-6 ${
              selectedCourse === 1
                ? "bg-orange-500 hover:bg-orange-600 text-white"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }`}
          >
            1 курс
          </Button>
          <Button
            onClick={() => setSelectedCourse(2)}
            className={`font-semibold text-md px-6 ${
              selectedCourse === 2
                ? "bg-orange-500 hover:bg-orange-600 text-white"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }`}
          >
            2 курс
          </Button>
        </div>

        <div className="flex w-full md:w-auto gap-2">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="outline"
                className="gap-2 bg-secondary/30 border-border/50"
              >
                <Filter className="h-4 w-4" />
                {selectedSubjects.length > 0 && (
                  <Badge
                    variant="secondary"
                    className="ml-1 h-5 px-1.5 bg-orange-500/10 text-orange-600"
                  >
                    {selectedSubjects.length}
                  </Badge>
                )}
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-56">
              <DropdownMenuLabel>Выберите предметы</DropdownMenuLabel>
              <DropdownMenuSeparator />
              {ALL_SUBJECTS.map((subject) => (
                <DropdownMenuCheckboxItem
                  key={subject}
                  checked={selectedSubjects.includes(subject)}
                  onCheckedChange={() => toggleSubject(subject)}
                >
                  {subject}
                </DropdownMenuCheckboxItem>
              ))}
              {selectedSubjects.length > 0 && (
                <>
                  <DropdownMenuSeparator />
                  <div className="p-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      className="w-full h-8 text-xs justify-center"
                      onClick={() => setSelectedSubjects([])}
                    >
                      Сбросить фильтры
                    </Button>
                  </div>
                </>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
          <div className="relative w-full md:w-80">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Поиск..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9 bg-secondary/30 border-border/50 focus-visible:ring-orange-500"
            />
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredMaterials.length > 0 ? (
          filteredMaterials.map((material) => (
            <Card
              key={material.id}
              // 1. ВЕРНУЛИ flex flex-col h-full, чтобы карточка растягивалась и футер был внизу
              className="flex flex-col h-full border-border/50 bg-card hover:border-orange-500/50 transition-colors cursor-pointer group"
            >
              {/* 2. Убрали h-[full]. Сделали pb-2 (маленький отступ снизу) */}
              <CardHeader className="pb-2">
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
                {" "}
                {/* Чуть добавили отступа перед футером для красоты */}
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
            Ничего не найдено
          </div>
        )}
      </div>
    </div>
  );
};
