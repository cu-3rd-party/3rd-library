import { useState } from "react";

import { SingleSelect } from "@/common/molecules/ui/SingleSelect";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import { DIFFICULTY_BADGES } from "@/constants";
import { cn } from "@/lib/utils";
import { Difficulty } from "@/models";


type DifficultySelectorProps = {
  difficulty: Difficulty;
  onSelect: (difficulty: Difficulty) => void;
};

export const DifficultySelector = ({
  difficulty,
  onSelect,
}: DifficultySelectorProps) => {
  const [open, setOpen] = useState(false);
  const getLevelData = (value: Difficulty) =>
    DIFFICULTY_BADGES.find((l) => l.value === value);

  return (
    <div className="space-y-3">
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