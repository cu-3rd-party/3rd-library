import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { MaterialCard, MaterialTopPanel } from "@/common/molecules/Materials";
import { cn } from "@/lib/utils";
import { Material } from "@/models";
import { useFilterSortStore } from "@/store";

type MaterialsSectionProps = {
  materials: Material[];
  className?: string;
};

export const MaterialsSection = ({
  materials,
  className,
}: MaterialsSectionProps) => {
  const navigate = useNavigate();
  const { filterState, sortState } = useFilterSortStore();
  const [searchQuery, setSearchQuery] = useState("");

  const handleCardClick = (id: string) => {
    navigate(`/materials/${id}`);
  };

  const sortedMaterials = materials
    .filter((item) => {
      const matchSearch = item.title
        .toLowerCase()
        .includes(searchQuery.toLowerCase());
      if (!matchSearch) return false;
      const matchCourses =
        filterState.courses.length === 0 ||
        filterState.courses.some((course) => item.courses.includes(course));
      if (!matchCourses) return false;
      const matchSubjects =
        filterState.subjects.length === 0 ||
        filterState.subjects.some((subject) => item.subjects.includes(subject));
      if (!matchSubjects) return false;
      const matchTypes =
        filterState.types.length === 0 || filterState.types.includes(item.type);
      if (!matchTypes) return false;
      const matchDifficulty =
        filterState.difficulties.length === 0 ||
        filterState.difficulties.includes(item.difficulty);
      return matchDifficulty;
    })
    .sort((a, b) => {
      let comparison = 0;

      switch (sortState.sortBy) {
        case "date": {
          // TODO: iso date
          const parseDate = (dateStr: string) => {
            const [day, month, year] = dateStr.split(".");
            return new Date(`${year}-${month}-${day}`).getTime();
          };
          comparison = parseDate(a.pubDate) - parseDate(b.pubDate);
          break;
        }
        case "title":
          comparison = a.title.localeCompare(b.title);
          break;
        case "subject":
          comparison = (a.subjects[0] || "").localeCompare(b.subjects[0] || "");
          break;
      }

      return sortState.order === "increasing" ? comparison : -comparison;
    });

  return (
    <div className={cn("space-y-4", className)}>
      <MaterialTopPanel
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
      />
      <div className="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2 lg:gap-6">
        {sortedMaterials.length > 0 ? (
          sortedMaterials.map((material) => (
            <MaterialCard
              key={material.id}
              material={material}
              onClick={handleCardClick}
              showAuthor={true}
            />
          ))
        ) : (
          <div className="col-span-full text-center py-20 bg-card rounded-xl border border-border">
            Ничего не найдено
          </div>
        )}
      </div>
    </div>
  );
};
