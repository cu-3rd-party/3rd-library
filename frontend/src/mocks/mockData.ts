import { Material } from "@/models/material";
import { User } from "@/models/user";

const DEFAULT_USER_ID = "3fa85f64-5717-4562-b3fc-2c963f66afa6";

export const MOCK_MATERIALS: Material[] = [
  {
    id: "a3f5c8d2-1e4b-4c9a-8f2d-6b7e9a3c5d1f",
    courses: ["1"],
    title: "Привет",
    pubDate: "02.02.2026",
    description:
      "Разбираем",
    authorId: DEFAULT_USER_ID,
    subjects: ["Матан", "Линал", "Кумир"],
    type: "shortread",
    difficulty: "blue",
    authorName: "Даниил Матанович"
  },
  {
    id: "b7e2a9f4-3d6c-4a8b-9e1f-5c7d8a2b4e6g",
    courses: ["1"],
    title: "Линейные операторы и их страхи",
    pubDate: "28.01.2026",
    description:
      "Разбираем",
    subjects: ["Линал"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "red",
    authorName: "Даниил Матанович"
  },
  {
    id: "c9d4b6e8-5f7a-4b2c-8d3e-9a1f6c4b7e5h",
    courses: ["2"],
    title: "Дифференциальные уравнения: Метод вариации постоянной",
    pubDate: "15.01.2026",
    description:
      "Разбираем",
    subjects: ["Диффуры"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "black",
    authorName: "Даниил Матанович"
  },
  {
    id: "d2f5a8c3-6e9b-4d7a-9f1c-8b3e5a6d4c2i",
    courses: ["1", "2"],
    title: "Интегралы: Методы замены переменной",
    pubDate: "10.01.2026",
    description:
      "Разбираем",
    subjects: ["Матан"],
    type: "longread",
    authorId: DEFAULT_USER_ID,
    difficulty: "blue",
    authorName: "Даниил Матанович"
  },
  {
    id: "e8a3c5d7-9f2b-4e6a-8c1d-7b4f9a2e5c3j",
    courses: ["2"],
    title: "Ряды Фурье: Разложение функций",
    pubDate: "05.12.2025",
    description:
      "Зачем раскладывать функции в ряд? Условия Дирихле. Четные и нечетные функции. Как это применяется в реальной жизни (спойлер: обработка сигналов).",
    subjects: ["Матан"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "red",
    authorName: "Даниил Матанович"
  },
  {
    id: "f1c7b9e4-8d3a-4f5c-9e2b-6a8d4c7e1f5k",
    courses: ["1"],
    title: "Аналитическая геометрия: Кривые второго порядка",
    pubDate: "20.11.2025",
    description:
      "Эллипс, гипербола, парабола. Как привести уравнение к каноническому виду и построить график. Инварианты кривых второго порядка.",
    subjects: ["Ангем"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "none",
    authorName: "Даниил Матанович"
  },
  {
    id: "13f5c8d2-1e4b-4c9a-8f2d-6b7e9a3c5d1f",
    courses: ["1"],
    title: "Полный гайд по Пределам: от Epsilon до бесконечности",
    pubDate: "02.02.2026",
    description:
      "Разбираем определение предела по Коши и Гейне. Почему 0/0 это не всегда единица, и как пользоваться правилом Лопиталя, чтобы вас не выгнали с экзамена. Много примеров с разбором.",
    authorId: DEFAULT_USER_ID,
    subjects: ["Матан"],
    type: "shortread",
    difficulty: "blue",
    authorName: "Даниил Матанович"
  },
  {
    id: "27e2a9f4-3d6c-4a8b-9e1f-5c7d8a2b4e6g",
    courses: ["1"],
    title: "Линейные операторы и их страхи",
    pubDate: "28.01.2026",
    description:
      "Что такое ядро и образ? Как найти собственные векторы и собственные значения, не сойдя с ума. Матрица перехода и изменение базиса простыми словами. Включает шпаргалку по свойствам.",
    subjects: ["Линал"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "red",
    authorName: "Даниил Матанович"
  },
  {
    id: "39d4b6e8-5f7a-4b2c-8d3e-9a1f6c4b7e5h",
    courses: ["2"],
    title: "Дифференциальные уравнения: Метод вариации постоянной",
    pubDate: "15.01.2026",
    description:
      "Секретная техника решения неоднородных уравнений. Пошаговый алгоритм Лагранжа. Разбор типовых задач из контрольных прошлых лет.",
    subjects: ["Диффуры"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "black",
    authorName: "Даниил Матанович"
  },
  {
    id: "42f5a8c3-6e9b-4d7a-9f1c-8b3e5a6d4c2i",
    courses: ["1"],
    title: "Интегралы: Методы замены переменной",
    pubDate: "10.01.2026",
    description:
      "Как выбрать правильную замену и не запутаться в дифференциалах. Тригонометрические подстановки, которые спасают жизни. Таблица основных интегралов прилагается.",
    subjects: ["Матан"],
    type: "longread",
    authorId: DEFAULT_USER_ID,
    difficulty: "blue",
    authorName: "Даниил Матанович"
  },
  {
    id: "58a3c5d7-9f2b-4e6a-8c1d-7b4f9a2e5c3j",
    courses: ["2"],
    title: "Ряды Фурье: Разложение функций",
    pubDate: "05.12.2025",
    description:
      "Зачем раскладывать функции в ряд? Условия Дирихле. Четные и нечетные функции. Как это применяется в реальной жизни (спойлер: обработка сигналов).",
    subjects: ["Матан"],
    type: "shortread",
    authorId: DEFAULT_USER_ID,
    difficulty: "red",
    authorName: "Даниил Матанович"
  },
  {
    id: "61c7b9e4-8d3a-4f5c-9e2b-6a8d4c7e1f5k",
    courses: ["1"],
    title: "Аналитическая геометрия: Кривые второго порядка",
    pubDate: "20.11.2025",
    description:
      "Эллипс, гипербола, парабола. Как привести уравнение к каноническому виду и построить график. Инварианты кривых второго порядка.",
    subjects: ["Ангем"],
    type: "shortread",
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