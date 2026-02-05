import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { MaterialCard, MaterialSearchBar } from "@/common/molecules/Materials";
import { cn } from "@/lib/utils";
import { defaultFilterState, FilterState, Material } from "@/models/material";

type MaterialsSectionProps = {
  materials: Material[];
  className?: string;
};

export const MaterialsSection = ({
  materials,
  className,
}: MaterialsSectionProps) => {
  const navigate = useNavigate();
  const [filters, setFilters] = useState<FilterState>(defaultFilterState);
  const [searchQuery, setSearchQuery] = useState("");

  const handleCardClick = (id: string) => {
    navigate(`/materials/${id}`);
  };

  const filteredMaterials = materials.filter((item) => {
    const matchSearch = item.title.toLowerCase().includes(searchQuery.toLowerCase());
    if (!matchSearch) return false;
    const matchCourses = filters.courses.length === 0 || filters.courses.some(course => item.courses.includes(course))
    if (!matchCourses) return false;
    const matchSubjects = filters.subjects.length === 0 || filters.subjects.some(subject => item.subjects.includes(subject))
    if (!matchSubjects) return false;
    const matchTypes = filters.types.length === 0 || filters.types.includes(item.type)
    if (!matchTypes) return false;
    const matchDifficulty = filters.difficulties.length === 0 || filters.difficulties.includes(item.difficulty)
    return matchDifficulty
  });

  return (
    <div className={cn("space-y-4 lg:space-y-8", className)}>
      <MaterialSearchBar
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        filters={filters}
        onFilterChange={(key, value) => setFilters(prev => ({...prev, [key]: value}))}
      />
      <div className="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2 lg:gap-6">
        {filteredMaterials.length > 0 ? (
          filteredMaterials.map((material) => (
            <MaterialCard
              key={material.id}
              material={material}
              onClick={handleCardClick}
              showAuthor={true}
            />
          ))
        ) : (
          <div className="col-span-full text-center py-20 bg-card rounded-xl border border-dashed border-border">
            Ничего не найдено
          </div>
        )}
      </div>
    </div>
  );
};
