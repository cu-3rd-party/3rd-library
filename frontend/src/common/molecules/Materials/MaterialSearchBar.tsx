import { Search, Filter, Check } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "@/components/ui/command";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { SUBJECTS } from "@/constants";
import { cn } from "@/lib/utils";


type MaterialSearchBarProps = {
  searchQuery: string;
  onSearchChange: (value: string) => void;
  selectedSubjects: string[];
  onSubjectToggle: (subject: string) => void;
  onResetSubjects: () => void;
};

export const MaterialSearchBar = ({
  searchQuery,
  onSearchChange,
  selectedSubjects,
  onSubjectToggle,
  onResetSubjects,
}: MaterialSearchBarProps) => (
  <div className="flex w-full lg:w-auto gap-2">
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" className="gap-2 border-border">
          <Filter className="h-4 w-4" />
          {selectedSubjects.length > 0 && (
            <Badge
              variant="secondary"
              className="ml-1 h-5 px-1.5"
            >
              {selectedSubjects.length}
            </Badge>
          )}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-56 p-0">
        <Command>
          <CommandInput placeholder="Поиск предметов..." />
          <CommandList>
            <CommandEmpty>Предметы не найдены</CommandEmpty>
            <CommandGroup>
              {SUBJECTS.map((subject) => (
                <CommandItem
                  key={subject}
                  value={subject}
                  onSelect={() => onSubjectToggle(subject)}
                >
                  <Check
                    className={cn(
                      "mr-2 h-4 w-4",
                      selectedSubjects.includes(subject) ? "opacity-100" : "opacity-0",
                    )}
                  />
                  {subject}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
          {selectedSubjects.length > 0 && (
            <>
              <DropdownMenuSeparator />
              <div className="p-1">
                <Button
                  variant="ghost"
                  size="lg"
                  className="w-full h-8 text-sm justify-center bg-input hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-none"
                  onClick={onResetSubjects}
                >
                  Сбросить фильтры
                </Button>
              </div>
            </>
          )}
        </Command>
      </DropdownMenuContent>
    </DropdownMenu>

    <div className="relative w-full lg:w-80">
      <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
      <Input
        placeholder="Поиск..."
        value={searchQuery}
        onChange={(e) => onSearchChange(e.target.value)}
        className="pl-9 border-border"
      />
    </div>
  </div>
);