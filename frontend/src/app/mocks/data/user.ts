import { User } from "@/entities/user/model";

export const MOCK_AUTHORS: User[] = [
  {
    id: "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d",
    name: "Матан Матанович",
    bio: "Профессор математического анализа с 15-летним стажем. Люблю пределы и интегралы.",
    verified: true,
  },
  {
    id: "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e",
    name: "Линал Линалович",
    bio: "Специалист по линейной алгебре. Матрицы - моя страсть.",
    verified: true,
  },
  {
    id: "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f",
    name: "Это Дима Трушин",
    bio: "Легендарный преподаватель ВШЭ. Автор культовых лекций.",
    verified: true,
  },
  {
    id: "d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a",
    name: "Валем",
    bio: "Студент. Делюсь конспектами. Ставьте лайки и подписывайтесь на канал.",
    verified: false,
  },
  {
    id: "e5f6a7b8-c9d0-4e1f-2a3b-4c5d6e7f8a9b",
    name: "Валеман",
    bio: "Просто математик. Просто хорошо объясняю.",
    verified: false,
  },
  {
    id: "f6a7b8c9-d0e1-4f2a-3b4c-5d6e7f8a9b0c",
    name: "Матан Матанович",
    bio: "Единственный и неповторимый.",
    verified: true,
  },
  {
    id: "a7b8c9d0-e1f2-4a3b-4c5d-6e7f8a9b0c1d",
    name: "Матан Матанович",
    bio: "Сын легендарного Матана Матановича.",
    verified: false,
  },
  {
    id: "b8c9d0e1-f2a3-4b4c-5d6e-7f8a9b0c1d2e",
    name: "Матан Матанович",
    bio: "Троюродный брат Линала Линаловича.",
    verified: true,
  },
];
