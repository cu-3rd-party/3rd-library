import { useState } from "react";

import { SingleSelect } from "@/common/molecules";
import { Label } from "@/components/ui/label";
import { DIFFICULTIES, DIFFICULTY_CONFIG } from "@/constants";
import { cn } from "@/lib/utils";
import { Difficulty } from "@/models";


type DifficultySelectorProps = {
  difficulty: Difficulty;
  onSelect: (difficulty: Difficulty) => void;
  className?: string;
};

export const DifficultySelector = ({
  difficulty,
  onSelect,
  className
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
        placeholder="Выберите уровень сложности..."
        renderItem={(d) => <span>{DIFFICULTY_CONFIG[d]?.label_add || d}</span>}
        getItemKey={(d) => d}
        getItemValue={(d) => d}
      />
    </div>
  );
}