import { useState } from "react";

import { DIFFICULTY_CONFIG } from "@/entities/material/lib";
import { DIFFICULTIES, Difficulty } from "@/entities/material/model";
import { cn } from "@/shared/lib";
import { Label, SingleSelect } from "@/shared/ui";

type DifficultySelectorProps = {
  difficulty: Difficulty;
  onSelect: (difficulty: Difficulty) => void;
  className?: string;
};

export const DifficultySelector = ({
  difficulty,
  onSelect,
  className,
}: DifficultySelectorProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className={cn("space-y-3", className)}>
      <Label className="text-lg font-semibold">Уровень сложности</Label>
      <SingleSelect
        open={open}
        onOpenChange={setOpen}
        selectedItem={difficulty || null}
        onSelect={onSelect}
        items={DIFFICULTIES}
        renderItem={(d) => <span>{DIFFICULTY_CONFIG[d]?.label_add || d}</span>}
        getItemKey={(d) => d}
        getItemValue={(d) => d}
      />
    </div>
  );
};
