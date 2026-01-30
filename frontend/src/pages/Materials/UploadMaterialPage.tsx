import { useState, useRef } from "react";
import { X, FileText, CircleAlert, SendHorizonal, Check, ChevronsUpDown } from "lucide-react";
import { cn } from "@/lib/utils"; // Утилита для объединения классов (стандарт shadcn)

import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";

const COURSES = ["1 курс", "2 курс"];
const SUBJECTS = ["Матан", "Линал", "Диффуры", "Английский", "Алгоритмы", "Физика"];

export const UploadMaterialPage = () => {
  const [file, setFile] = useState<File | null>(null);
  // Состояния для множественного выбора
  const [selectedCourses, setSelectedCourses] = useState<string[]>([]);
  const [selectedSubjects, setSelectedSubjects] = useState<string[]>([]);

  // Состояния открытия выпадающих списков
  const [openCourses, setOpenCourses] = useState(false);
  const [openSubjects, setOpenSubjects] = useState(false);

  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const selectedFile = e.target.files[0];
      if (selectedFile.type === "application/pdf") {
        setFile(selectedFile);
      } else {
        alert("Пожалуйста, выберите файл формата .pdf");
      }
    }
  };

  const handleRemoveFile = () => {
    setFile(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  };

  const toggleSelection = (
    item: string,
    currentList: string[],
    setList: (list: string[]) => void
  ) => {
    if (currentList.includes(item)) {
      setList(currentList.filter((i) => i !== item));
    } else {
      setList([...currentList, item]);
    }
  };

  return (
    <div className="container mx-auto max-w-screen-xl px-4 md:px-8 md:py-4 space-y-4 md:space-y-8">
      <div className="space-y-3">
        <Label htmlFor="title" className="text-lg font-semibold">
          Название
        </Label>
        <Input
          id="title"
          placeholder="Например: Полный курс лекций по матанализу"
          className="bg-background text-base md:text-lg py-6"
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">

        <div className="space-y-3">
          <Label className="text-lg font-semibold">Курсы</Label>
          <Popover open={openCourses} onOpenChange={setOpenCourses}>
            <PopoverTrigger asChild>
              <Button
                variant="outline"
                role="combobox"
                aria-expanded={openCourses}
                className="w-full justify-between h-auto min-h-[3rem] bg-background px-3 py-2 hover:bg-background"
              >
                <div className="flex flex-wrap gap-1 items-center">
                  {selectedCourses.length > 0 ? (
                    selectedCourses.map((course) => (
                      <Badge variant="secondary" key={course} className="mr-1">
                        {course}
                      </Badge>
                    ))
                  ) : (
                    <span className="text-muted-foreground font-normal text-base">
                      Выберите курсы...
                    </span>
                  )}
                </div>
                <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] p-0" align="start">
              <Command>
                <CommandList>
                  <CommandEmpty>Курс не найден.</CommandEmpty>
                  <CommandGroup>
                    {COURSES.map((course) => (
                      <CommandItem
                        key={course}
                        value={course}
                        onSelect={() => toggleSelection(course, selectedCourses, setSelectedCourses)}
                      >
                        <Check
                          className={cn(
                            "mr-2 h-4 w-4",
                            selectedCourses.includes(course) ? "opacity-100" : "opacity-0"
                          )}
                        />
                        {course}
                      </CommandItem>
                    ))}
                  </CommandGroup>
                </CommandList>
              </Command>
            </PopoverContent>
          </Popover>
        </div>

        <div className="space-y-3">
          <Label className="text-lg font-semibold">Предметы</Label>
          <Popover open={openSubjects} onOpenChange={setOpenSubjects}>
            <PopoverTrigger asChild>
              <Button
                variant="outline"
                role="combobox"
                aria-expanded={openSubjects}
                className="w-full justify-between h-auto min-h-[3rem] bg-background px-3 py-2 hover:bg-background"
              >
                <div className="flex flex-wrap gap-1 items-center">
                  {selectedSubjects.length > 0 ? (
                    selectedSubjects.map((subject) => (
                      <Badge variant="secondary" key={subject} className="mr-1">
                        {subject}
                      </Badge>
                    ))
                  ) : (
                    <span className="text-muted-foreground font-normal text-base">
                      Выберите предметы...
                    </span>
                  )}
                </div>
                <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] p-0" align="start">
              <Command>
                 {/* Если предметов много, можно добавить поиск */}
                 <CommandInput placeholder="Найти предмет..." />
                <CommandList>
                  <CommandEmpty>Предмет не найден.</CommandEmpty>
                  <CommandGroup>
                    {SUBJECTS.map((subject) => (
                      <CommandItem
                        key={subject}
                        value={subject}
                        onSelect={() => toggleSelection(subject, selectedSubjects, setSelectedSubjects)}
                      >
                        <Check
                          className={cn(
                            "mr-2 h-4 w-4",
                            selectedSubjects.includes(subject) ? "opacity-100" : "opacity-0"
                          )}
                        />
                        {subject}
                      </CommandItem>
                    ))}
                  </CommandGroup>
                </CommandList>
              </Command>
            </PopoverContent>
          </Popover>
        </div>
      </div>

      {/* 3. Описание */}
      <div className="space-y-3">
        <Label htmlFor="description" className="text-lg font-semibold">
          Описание
        </Label>
        <Textarea
          id="description"
          placeholder="Краткое содержание..."
          className="bg-background min-h-[150px] text-base resize-y"
        />
      </div>

      {/* 4. Файл */}
      <div className="space-y-3">
        <Label className="text-lg font-semibold">Файл материала</Label>
        <input
          type="file"
          accept=".pdf"
          ref={fileInputRef}
          onChange={handleFileChange}
          className="hidden"
        />

        {!file ? (
          <div
            onClick={() => fileInputRef.current?.click()}
            className="border-2 border-dashed border-border/60 hover:border-primary/50 hover:bg-muted/30 rounded-lg p-6 md:p-8 flex flex-col items-center justify-center cursor-pointer transition-all gap-3 text-center touch-manipulation"
          >
            <div className="p-3 bg-muted rounded-full">
              <FileText className="h-6 w-6 text-muted-foreground" />
            </div>
            <div>
              <span className="font-semibold text-primary block mb-1">Загрузить файл</span>
              <span className="text-xs text-muted-foreground">PDF</span>
            </div>
          </div>
        ) : (
          <div className="flex items-center gap-3 p-4 border border-border rounded-lg bg-background overflow-hidden">
            <div className="bg-red-500/10 p-2 rounded shrink-0">
              <FileText className="h-6 w-6 text-red-500" />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <span className="font-medium text-sm truncate">{file.name}</span>
              <span className="text-xs text-muted-foreground">
                {(file.size / 1024 / 1024).toFixed(2)} MB
              </span>
            </div>
            <Button variant="ghost" size="icon" onClick={handleRemoveFile} className="shrink-0">
              <X className="h-5 w-5" />
            </Button>
          </div>
        )}
      </div>

      {/* 5. Футер */}
      <div className="pt-6 flex flex-col-reverse md:flex-row items-center justify-end gap-4 md:gap-6">
        <TooltipProvider>
          <Tooltip delayDuration={300}>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-2 cursor-help text-muted-foreground hover:text-orange-500 transition-colors py-2 md:py-0">
                <span className="md:hidden text-sm">О модерации</span>
                <CircleAlert className="h-6 w-6" />
              </div>
            </TooltipTrigger>
            <TooltipContent side="top" className="max-w-[200px] text-center md:text-left">
              <p>
                Все материалы проходят предварительную проверку модератором
                перед публикацией.
              </p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>

        <Button
          size="lg"
          className="w-full md:w-auto bg-orange-500 text-primary-foreground hover:bg-orange-600 font-bold px-4 text-md shadow-md transition-transform active:scale-95 flex items-center justify-center "
          onClick={() => alert("Отправлено!")}
        >
          Отправить
          <SendHorizonal className="h-5 w-5" />
        </Button>
      </div>
    </div>
  );
};