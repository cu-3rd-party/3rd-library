import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { CourseSelector } from "@/common/atoms";
import { MaterialCard, MaterialSearchBar } from "@/common/molecules/Materials";
import { Material } from "@/common/molecules/Materials/MaterialCard.tsx";
import { cn } from "@/lib/utils";

type MaterialsSectionProps = {
  materials: Material[];
  className?: string;
};

export const MaterialsSection = ({
  materials,
  className,
}: MaterialsSectionProps) => {
  const navigate = useNavigate();

  const [selectedCourse, setSelectedCourse] = useState<1 | 2>(1);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedSubjects, setSelectedSubjects] = useState<string[]>([]);

  const handleSubjectToggle = (subject: string) => {
    setSelectedSubjects((prev) =>
      prev.includes(subject)
        ? prev.filter((s) => s !== subject)
        : [...prev, subject],
    );
  };

  const handleCardClick = (id: number) => {
    navigate(`/materials/${id}`);
  };

  const filteredMaterials = materials.filter((item) => {
    const matchCourse = item.course === selectedCourse;

    const matchSearch =
      item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.previewText.toLowerCase().includes(searchQuery.toLowerCase());

    const matchSubject =
      selectedSubjects.length === 0 ||
      item.tags.some((tag) => selectedSubjects.includes(tag));

    return matchCourse && matchSearch && matchSubject;
  });

  return (
    <div className={cn("space-y-8", className)}>
      {/* Upper Control Bar (Molecules) */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <CourseSelector
          selectedCourse={selectedCourse}
          onSelect={setSelectedCourse}
        />

        <MaterialSearchBar
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          selectedSubjects={selectedSubjects}
          onSubjectToggle={handleSubjectToggle}
          onResetSubjects={() => setSelectedSubjects([])}
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredMaterials.length > 0 ? (
          filteredMaterials.map((material) => (
            <MaterialCard
              key={material.id}
              material={material}
              onClick={handleCardClick}
            />
          ))
        ) : (
          <div className="col-span-full text-center py-20 text-muted-foreground bg-secondary/10 rounded-xl border border-dashed border-border/50">
            Ничего не найдено
          </div>
        )}
      </div>
    </div>
  );
};
