import { getCourseName, TYPE_CONFIG } from "@/entities/material/lib";
import { MaterialDetailsResponse } from "../model";

interface Props {
  materialDetails: MaterialDetailsResponse;
}

export const MaterialDetailsMeta = ({ materialDetails }: Props) => {
  const typeData = TYPE_CONFIG[materialDetails.type];

  return (
    <>
      <div className="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm lg:text-base font-medium text-foreground">
        <p>{getCourseName(materialDetails.courses)}</p>
        {typeData && <p>{typeData.label}</p>}
      </div>
      <p className="max-w-4xl whitespace-pre-wrap text-sm leading-relaxed text-muted-foreground">
        {materialDetails.description}
      </p>
    </>
  );
};
