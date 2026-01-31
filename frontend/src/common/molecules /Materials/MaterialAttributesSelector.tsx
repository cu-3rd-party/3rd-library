import { Check, ChevronsUpDown } from "lucide-react";
import { useState } from "react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
} from "@/components/ui/command";
import { Label } from "@/components/ui/label";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { DIFFICULTY_LEVELS } from "@/const/difficulty";
import { cn } from "@/lib/utils";

const COURSES = ["1 курс", "2 курс"];
const SUBJECTS = ["Матан", "Линал", "Диффуры", "Английский", "Алгоритмы", "Физика"];

type AttributesSelectorProps = {
  courses: string[];
  subjects: string[];
  difficulties: string[];
  onToggleCourse: (val: string) => void;
  onToggleSubject: (val: string) => void;
  onToggleDifficulty: (val: string) => void;
};

export const MaterialAttributesSelector = ({
  courses,
  subjects,
  difficulties,
  onToggleCourse,
  onToggleSubject,
  onToggleDifficulty,
}: AttributesSelectorProps) => {
  const [openCourses, setOpenCourses] = useState(false);
  const [openSubjects, setOpenSubjects] = useState(false);
  const [openDifficulty, setOpenDifficulty] = useState(false);

  const getLevelData = (value: string) => DIFFICULTY_LEVELS.find((l) => l.value === value);

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 md:gap-6">
        <div className="space-y-3">
        <Label className="text-lg font-semibold">Курсы</Label>
        <Popover open={openCourses} onOpenChange={setOpenCourses}>
          <PopoverTrigger asChild>
            <Button variant="outline" role="combobox" aria-expanded={openCourses} className="w-full justify-between h-auto min-h-[3rem] bg-background px-3 py-2 hover:bg-background">
              <div className="flex flex-wrap gap-1 items-center">
                {courses.length > 0 ? (
                  courses.map((c) => <Badge variant="secondary" key={c} className="mr-1">{c}</Badge>)
                ) : (
                  <span className="text-muted-foreground font-normal text-base">Выберите курсы...</span>
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
                  {COURSES.map((c) => (
                    <CommandItem key={c} value={c} onSelect={() => onToggleCourse(c)}>
                      <Check className={cn("mr-2 h-4 w-4", courses.includes(c) ? "opacity-100" : "opacity-0")} />
                      {c}
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
            <Button variant="outline" role="combobox" aria-expanded={openSubjects} className="w-full justify-between h-auto min-h-[3rem] bg-background px-3 py-2 hover:bg-background">
              <div className="flex flex-wrap gap-1 items-center">
                {subjects.length > 0 ? (
                  subjects.map((s) => <Badge variant="secondary" key={s} className="mr-1">{s}</Badge>)
                ) : (
                  <span className="text-muted-foreground font-normal text-base">Выберите предметы...</span>
                )}
              </div>
              <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[300px] p-0" align="start">
            <Command>
              <CommandInput placeholder="Найти предмет..." />
              <CommandList>
                <CommandEmpty>Предмет не найден.</CommandEmpty>
                <CommandGroup>
                  {SUBJECTS.map((s) => (
                    <CommandItem key={s} value={s} onSelect={() => onToggleSubject(s)}>
                      <Check className={cn("mr-2 h-4 w-4", subjects.includes(s) ? "opacity-100" : "opacity-0")} />
                      {s}
                    </CommandItem>
                  ))}
                </CommandGroup>
              </CommandList>
            </Command>
          </PopoverContent>
        </Popover>
      </div>

      <div className="space-y-3">
        <Label className="text-lg font-semibold">Сложность</Label>
        <Popover open={openDifficulty} onOpenChange={setOpenDifficulty}>
          <PopoverTrigger asChild>
            <Button variant="outline" role="combobox" aria-expanded={openDifficulty} className="w-full justify-between h-auto min-h-[3rem] bg-background px-3 py-2 hover:bg-background">
              <div className="flex flex-wrap gap-1 items-center">
                {difficulties.length > 0 ? (
                  difficulties.map((val) => {
                    const level = getLevelData(val);
                    return <Badge key={val} className={cn("mr-1 border", level?.badgeClass)}>{level?.label || val}</Badge>;
                  })
                ) : (
                  <span className="text-muted-foreground font-normal text-base">Выберите уровни...</span>
                )}
              </div>
              <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[300px] p-0" align="start">
            <Command>
              <CommandList>
                <CommandEmpty>Уровень не найден.</CommandEmpty>
                <CommandGroup>
                  {DIFFICULTY_LEVELS.map((level) => (
                    <CommandItem key={level.value} value={level.value} onSelect={() => onToggleDifficulty(level.value)}>
                      <Check className={cn("mr-2 h-4 w-4", difficulties.includes(level.value) ? "opacity-100" : "opacity-0")} />
                      <div className="flex items-center gap-2">
                        <div className={cn("w-3 h-3 rounded-full", level.indicatorClass)} />
                        {level.label}
                      </div>
                    </CommandItem>
                  ))}
                </CommandGroup>
              </CommandList>
            </Command>
          </PopoverContent>
        </Popover>
      </div>
    </div>
  );
};