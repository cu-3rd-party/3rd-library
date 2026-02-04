import { useState } from "react";

import { SingleSelect } from "@/common/molecules/ui/SingleSelect";
import { Label } from "@/components/ui/label";
import { DIFFICULTY_BADGES } from "@/constants";
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
  const getLevelData = (value: Difficulty) =>
    DIFFICULTY_BADGES.find((l) => l.value === value);

  return (
    <div className={`space-y-3 ${className || ''}`}>
      <Label className="text-lg font-semibold">Уровень сложности</Label>
      <SingleSelect
        open={open}
        onOpenChange={setOpen}
        selectedItem={difficulty || null}
        onSelect={onSelect}
        items={DIFFICULTY_BADGES.map((d) => d.value)}
        placeholder="Выберите уровень сложности..."
        renderItem={(d) => {
          const level = getLevelData(d);
          return <span>{level?.label || d}</span>;
        }}
        getItemKey={(d) => d}
        getItemValue={(d) => d}
      />
    </div>
  );
}