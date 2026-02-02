import { Material } from "./material";
import { UUID } from "./types";

export interface User {
  id: UUID;
  name: string;
  bio: string;
  verified: boolean;
  materials: Material[];
}