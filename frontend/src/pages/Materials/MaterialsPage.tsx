import { Material } from "@/common/molecules/Materials/MaterialCard.tsx";
import {MaterialsSection} from "@/common/organisms";

const MOCK_ALL_MATERIALS: Material[] = [
  {
    id: 1,
    course: 1,
    title: "Полный сборник лонгридов по матану",
    date: "29.01.2026",
    previewText: "9. НЕВЫРОЖДЕННЫЕ МАТРИЦЫ. Нет, не начнем...",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
    difficulty: "red",
  },
  {
    id: 2,
    course: 1,
    title: "Введение в Линейную Алгебру",
    date: "28.01.2026",
    previewText: "Линейное пространство, базис, размерность...",
    tags: ["Линал", "1 курс"],
    author: "Алексей Векторов",
    difficulty: "blue",
  },
  {
    id: 3,
    course: 2,
    title: "Дифференциальные уравнения: основы",
    date: "15.01.2026",
    previewText: "Диффуры — это не страшно...",
    tags: ["Диффуры", "2 курс"],
    author: "Мария Интегралова",
    difficulty: "none",
  },
  {
    id: 4,
    course: 1,
    title: "Пределы последовательностей",
    date: "30.01.2026",
    previewText: "Определение предела по Коши...",
      tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
    difficulty: "black",
  },
];

export const MaterialsPage = () => {
  return (
    <div className="container w-5/6 m-auto max-w-screen-2xl py-6">
      <h1 className="text-3xl font-bold mb-6">Все материалы</h1>
      <MaterialsSection materials={MOCK_ALL_MATERIALS} />
    </div>
  );
};
