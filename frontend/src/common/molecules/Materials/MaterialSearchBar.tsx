import { Search, Filter } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";

export const ALL_SUBJECTS = ["Матан", "Линал", "Диффуры"];

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
}: MaterialSearchBarProps) => {
  return (
    <div className="flex w-full md:w-auto gap-2">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="outline"
            className="gap-2 border-border"
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
              onCheckedChange={() => onSubjectToggle(subject)}
              onSelect={(e) => e.preventDefault()}
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
                  onClick={onResetSubjects}
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
          onChange={(e) => onSearchChange(e.target.value)}
          className="pl-9 border-border"
        />
      </div>
    </div>
  );
};
