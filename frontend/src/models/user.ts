import { Material } from "./material";
import { UUID } from "./types";

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
