import { Badge } from "@/components/ui/badge.tsx";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { DIFFICULTY_CONFIG } from "@/const/difficulty.ts";
import { cn } from "@/lib/utils.ts";

export type Material = {
  id: number;
  course: 1 | 2;
  title: string;
  date: string;
  previewText: string;
  tags: string[];
  author: string;
  difficulty: "none" | "blue" | "red" | "black";
};

type MaterialCardProps = {
  material: Material;
  onClick: (id: number) => void;
};

export const MaterialCard = ({ material, onClick }: MaterialCardProps) => {
  const difficultyData = DIFFICULTY_CONFIG[material.difficulty];

  return (
    <Card
      onClick={() => onClick(material.id)}
      className="flex flex-col h-full border-border/50 bg-card hover:border-orange-500/50 transition-colors cursor-pointer group"
    >
      <CardHeader className="pb-2">
        <CardTitle className="text-xl font-bold leading-tight group-hover:text-orange-500 transition-colors line-clamp-2">
          {material.title}
        </CardTitle>
        <p className="text-xs text-muted-foreground mt-1">{material.date}</p>
      </CardHeader>

      <CardContent className="flex-grow pt-0">
        <p className="text-sm text-muted-foreground line-clamp-6 leading-relaxed">
          {material.previewText}
        </p>
      </CardContent>

      <CardFooter className="flex items-center justify-between pt-4">
        <div className="flex flex-wrap gap-2">
          {/* Теги */}
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

          {material.difficulty !== "none" && (
            <Badge
              className={cn(
                "rounded-md px-2 py-0.5 text-xs font-normal",
                difficultyData.className,
              )}
            >
              {difficultyData.label}
            </Badge>
          )}
        </div>

        <span className="text-sm font-medium text-foreground ml-auto shrink-0">
          {material.author}
        </span>
      </CardFooter>
    </Card>
  );
};
