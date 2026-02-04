import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { DIFFICULTY_CONFIG } from "@/constants";
import { cn } from "@/lib/utils.ts";
import { Material } from "@/models/material";

type MaterialCardProps = {
  material: Material;
  onClick: (id: string) => void;
  showAuthor?: boolean;
};

export const MaterialCard = ({ material, onClick, showAuthor }: MaterialCardProps) => {
  const difficultyData = DIFFICULTY_CONFIG[material.difficulty];

  return (
    <Card
      onClick={() => onClick(material.id)}
      className="flex flex-col h-full border-border bg-card hover:border-ring transition-colors cursor-pointer group"
    >
      <CardHeader className="pb-2">
        <CardTitle className="text-xl font-bold leading-tight line-clamp-2">
          {material.title}
        </CardTitle>
        <p className="text-xs text-muted-foreground mt-1">{material.pubDate}</p>
      </CardHeader>

      <CardContent className="grow pt-0">
        <p className="text-sm text-muted-foreground line-clamp-6 leading-relaxed">
          {material.description}
        </p>
      </CardContent>

      <CardFooter className="flex items-center justify-between pt-4">
        <div className="flex flex-wrap gap-2">
          {[...material.subjects, material.type].map((tag, idx) => (
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
          
        {showAuthor && (
          <span className="text-sm font-medium text-foreground ml-auto shrink-0">
            {material.authorName}
          </span>
        )}
      </CardFooter>
    </Card>
  );
};
