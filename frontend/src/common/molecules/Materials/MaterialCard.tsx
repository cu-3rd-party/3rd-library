
import { MaterialBadge } from "@/common/atoms";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
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
      className="flex flex-col h-full border-border bg-card hover:border-ring transition-colors cursor-pointer group"
    >
      <CardHeader className="pb-2">
        <div className="flex flex-wrap lg:flex-nowrap gap-2 lg:mb-1">
          <div className="flex grow flex-wrap gap-2">
            <div className="flex justify-between gap-2 w-full md:w-auto">
              <MaterialBadge
                label={getCourseName(material.courses)}
                className="bg-orange-badge text-orange-badge-foreground"
              />
              <MaterialBadge
                label={typeData.label}
                className="md:hidden"
              />
            </div>
            
            <div className="flex gap-2">
              <MaterialBadge
                label={material.subjects[0]}
                className={difficultyData.className}
              />
              {material.subjects.length > 1 && (
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <MaterialBadge
                        label={`+${material.subjects.length - 1}`}
                        className={difficultyData.className}
                      />
                    </TooltipTrigger>
                    <TooltipContent>
                      <div className="flex flex-col gap-1">
                        {material.subjects.slice(1).map((subject, idx) => (
                          <span key={idx} className="text-xs">{subject}</span>
                        ))}
                      </div>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              )}
            </div>
          </div>

          
          <MaterialBadge
            label={typeData.label}
            className="hidden md:block md:ml-auto lg:self-start"
          />
        </div>

        <CardTitle className="text-sm md:text-lg lg:text-xl min-h-8.75 md:min-h-11.25 lg:min-h-12.5 font-bold leading-tight line-clamp-2 ">
          {material.title}
        </CardTitle>
      </CardHeader>

      <CardContent className="grow pt-0">
        <p className="text-xs md:text-sm text-muted-foreground line-clamp-3 leading-normal">
          {material.description}
        </p>
      </CardContent>

      <CardFooter className="flex flex-col md:flex-row items-start md:items-center md:justify-between pt-1 lg:pt-4 gap-1 md:gap-0">
        {showAuthor && (
          <span className="text-xs md:text-sm font-medium text-foreground shrink-0">
            {material.authorName}
          </span>
        )}
        <p className="text-xs md:text-sm text-muted-foreground md:ml-auto">{material.pubDate}</p>

      </CardFooter>
    </Card>
  );
};
