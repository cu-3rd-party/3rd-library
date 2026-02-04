import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { DIFFICULTY_CONFIG } from "@/constants";
import { TYPE_CONFIG } from "@/constants/materialType";
import { Material } from "@/models/material";
import { getCourseName } from "@/utils";

type MaterialCardProps = {
  material: Material;
  onClick: (id: string) => void;
  showAuthor?: boolean;
};

export const MaterialCard = ({ material, onClick, showAuthor }: MaterialCardProps) => {
  const difficultyData = DIFFICULTY_CONFIG[material.difficulty];
  const typeData = TYPE_CONFIG[material.type];
  return (
    <Card
      onClick={() => onClick(material.id)}
      className="flex flex-col h-full border-border bg-card hover:border-ring transition-colors cursor-pointer group gap-2"
    >
      <CardHeader className="pb-2">
        <div className="flex flex-wrap gap-2 mb-1">
          <Badge
            variant="secondary"
            className="rounded-md px-2 py-0.5 text-xs font-normal bg-orange-badge text-orange-badge-foreground"
          >
            {getCourseName(material.courses)}
          </Badge>
          {material.subjects.map((tag, idx) => (
            <Badge
              key={idx}
              variant="secondary"
              className={`rounded-md px-2 py-0.5 text-xs font-normal ${difficultyData.className}`}
            >
              {tag}
            </Badge>
          ))}
          <Badge
            variant="secondary"
            className="rounded-md px-2 py-0.5 text-xs font-normal ml-auto"
          >
            {typeData.label}
          </Badge>
        </div>

        <CardTitle className="text-base md:text-lg lg:text-xl font-bold leading-tight line-clamp-2 ">
          {material.title}
        </CardTitle>
      </CardHeader>

      <CardContent className="grow pt-0">
        <p className="text-xs md:text-sm text-muted-foreground line-clamp-3 leading-normal">
          {material.description}
        </p>
      </CardContent>

      <CardFooter className="flex items-center justify-between pt-4">
        <p className="text-xs md:text-sm text-muted-foreground">{material.pubDate}</p>
        {showAuthor && (
          <span className="text-sm font-medium text-foreground ml-auto shrink-0">
            {material.authorName}
          </span>
        )}
      </CardFooter>
    </Card>
  );
};
