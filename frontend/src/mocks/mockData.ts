import { Material } from "@/models/material";
import { User } from "@/models/user";

const DEFAULT_USER_ID = "3fa85f64-5717-4562-b3fc-2c963f66afa6";

export const MOCK_MATERIALS: Material[] = [
  {
    id: "a3f5c8d2-1e4b-4c9a-8f2d-6b7e9a3c5d1f",
    courses: [1],
    title: "Полный гайд по Пределам: от Epsilon до бесконечности",
    pubDate: "02.02.2026",
    description:
      "Разбираем определение предела по Коши и Гейне. Почему 0/0 это не всегда единица, и как пользоваться правилом Лопиталя, чтобы вас не выгнали с экзамена. Много примеров с разбором.",
    authorId: DEFAULT_USER_ID,
    subject: "Матан",
    type: "Шортрид",
    difficulty: "blue",
    authorName: "Даниил Матанович"
  },
  {
    id: "b7e2a9f4-3d6c-4a8b-9e1f-5c7d8a2b4e6g",
    courses: [1],
    title: "Линейные операторы и их страхи",
    pubDate: "28.01.2026",
    description:
      "Что такое ядро и образ? Как найти собственные векторы и собственные значения, не сойдя с ума. Матрица перехода и изменение базиса простыми словами. Включает шпаргалку по свойствам.",
    subject: "Линал",
    type: "Шортрид",
    authorId: DEFAULT_USER_ID,
    difficulty: "red",
    authorName: "Даниил Матанович"
  },
  {
    id: "c9d4b6e8-5f7a-4b2c-8d3e-9a1f6c4b7e5h",
    courses: [2],
    title: "Дифференциальные уравнения: Метод вариации постоянной",
    pubDate: "15.01.2026",
    description:
      "Секретная техника решения неоднородных уравнений. Пошаговый алгоритм Лагранжа. Разбор типовых задач из контрольных прошлых лет.",
    subject: "Диффуры",
    type: "Шортрид",
    authorId: DEFAULT_USER_ID,
    difficulty: "black",
    authorName: "Даниил Матанович"
  },
  {
    id: "d2f5a8c3-6e9b-4d7a-9f1c-8b3e5a6d4c2i",
    courses: [1],
    title: "Интегралы: Методы замены переменной",
    pubDate: "10.01.2026",
    description:
      "Как выбрать правильную замену и не запутаться в дифференциалах. Тригонометрические подстановки, которые спасают жизни. Таблица основных интегралов прилагается.",
    subject: "Матан",
    type: "Шортрид",
    authorId: DEFAULT_USER_ID,
    difficulty: "blue",
    authorName: "Даниил Матанович"
  },
  {
    id: "e8a3c5d7-9f2b-4e6a-8c1d-7b4f9a2e5c3j",
    courses: [2],
    title: "Ряды Фурье: Разложение функций",
    pubDate: "05.12.2025",
    description:
      "Зачем раскладывать функции в ряд? Условия Дирихле. Четные и нечетные функции. Как это применяется в реальной жизни (спойлер: обработка сигналов).",
    subject: "Матан",
    type: "Шортрид",
    authorId: DEFAULT_USER_ID,
    difficulty: "red",
    authorName: "Даниил Матанович"
  },
  {
    id: "f1c7b9e4-8d3a-4f5c-9e2b-6a8d4c7e1f5k",
    courses: [1],
    title: "Аналитическая геометрия: Кривые второго порядка",
    pubDate: "20.11.2025",
    description:
      "Эллипс, гипербола, парабола. Как привести уравнение к каноническому виду и построить график. Инварианты кривых второго порядка.",
    subject: "Ангем",
    type: "Шортрид",
    authorId: DEFAULT_USER_ID,
    difficulty: "none",
    authorName: "Даниил Матанович"
  },
];

export const MOCK_USER: User = {
  id: DEFAULT_USER_ID,
  name: "Даниил Матанович",
  bio: "Облизал весь матан с ног до головы. Объясняю сложные теоремы на пальцах (и иногда на котиках).",
  verified: true,
  materials: MOCK_MATERIALS
};
