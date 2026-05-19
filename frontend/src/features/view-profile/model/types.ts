import { Material } from "@/entities/material/model";
import { User } from "@/entities/user/model";

export type UserProfileResponse = {
  user: User;
  materials: {
    items: Material[];
    page: number;
    limit: number;
    total: number;
  };
};
