import { Material } from "@/entities/material/model";
import { UUID } from "@/shared/model";

export interface User {
  id: UUID;
  name: string;
  bio: string;
  image?: string | null;
  isEmailVerified?: boolean;
  verified?: boolean;
  materialsCount?: number;
  materials?: Material[];
}
