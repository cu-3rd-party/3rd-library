import { User } from "@/entities/user/model";
import { DEFAULT_USER_ID } from "@/shared/constants";

import { MOCK_MATERIALS } from "./material";

export const MOCK_USER: User = {
  id: DEFAULT_USER_ID,
  name: "Даниил Матанович",
  bio: "Облизал весь матан с ног до головы. Объясняю сложные теоремы на пальцах (и иногда на котиках).",
  verified: true,
  materials: MOCK_MATERIALS,
};
