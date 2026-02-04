import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { CourseSelector } from "@/common/atoms";
import { MaterialCard, MaterialSearchBar } from "@/common/molecules/Materials";
import { cn } from "@/lib/utils";
import { Material } from "@/models/material";

type MaterialsSectionProps = {
  materials: Material[];
  className?: string;
};

export const MaterialsSection = ({
  materials,
  className,
}: MaterialsSectionProps) => {
  const navigate = useNavigate();

  const [selectedCourse, setSelectedCourse] = useState<number>(1);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedSubjects, setSelectedSubjects] = useState<string[]>([]);

  const handleSubjectToggle = (subject: string) => {
    setSelectedSubjects((prev) =>
      prev.includes(subject)
        ? prev.filter((s) => s !== subject)
        : [...prev, subject],
    );
  };

  const handleCardClick = (id: string) => {
    navigate(`/materials/${id}`);
  };

  const filteredMaterials = materials.filter((item) => {
    const matchCourse = item.courses.includes(selectedCourse);

    const matchSearch =
      item.title.toLowerCase().includes(searchQuery.toLowerCase());

    const matchSubject =
      selectedSubjects.length === 0 ||
      selectedSubjects.some((subject) => item.subjects.includes(subject));

    return matchCourse && matchSearch && matchSubject;
  });

  return (
    <div className={cn("space-y-8", className)}>
      <div className="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4">
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

      <div className="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
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
