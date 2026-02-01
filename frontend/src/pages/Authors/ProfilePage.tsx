import { ShieldCheck, User } from "lucide-react";

import { Material } from "@/common/molecules/Materials/MaterialCard.tsx";
import { MaterialsSection } from "@/common/organisms";
import { Badge } from "@/components/ui/badge";

type UserProfile = {
  name: string;
  bio: string;
  avatar?: string;
  isVerified: boolean;
};

const MOCK_PROFILE: UserProfile = {
  name: "Даниил Матанович",
  bio: "Облизал весь матан с ног до головы. Объясняю сложные теоремы на пальцах (и иногда на котиках).",
  avatar: "/pwa-144x144.png",
  isVerified: true,
};

const AUTHOR_MATERIALS: Material[] = [
  {
    id: 101,
    course: 1,
    title: "Полный гайд по Пределам: от Epsilon до бесконечности",
    date: "02.02.2026",
    previewText:
      "Разбираем определение предела по Коши и Гейне. Почему 0/0 это не всегда единица, и как пользоваться правилом Лопиталя, чтобы вас не выгнали с экзамена. Много примеров с разбором.",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
    difficulty: "blue",
  },
  {
    id: 102,
    course: 1,
    title: "Линейные операторы и их страхи",
    date: "28.01.2026",
    previewText:
      "Что такое ядро и образ? Как найти собственные векторы и собственные значения, не сойдя с ума. Матрица перехода и изменение базиса простыми словами. Включает шпаргалку по свойствам.",
    tags: ["Линал", "1 курс"],
    author: "Даниил Матанович",
    difficulty: "red",
  },
  {
    id: 103,
    course: 2,
    title: "Дифференциальные уравнения: Метод вариации постоянной",
    date: "15.01.2026",
    previewText:
      "Секретная техника решения неоднородных уравнений. Пошаговый алгоритм Лагранжа. Разбор типовых задач из контрольных прошлых лет.",
    tags: ["Диффуры", "2 курс"],
    author: "Даниил Матанович",
    difficulty: "black",
  },
  {
    id: 104,
    course: 1,
    title: "Интегралы: Методы замены переменной",
    date: "10.01.2026",
    previewText:
      "Как выбрать правильную замену и не запутаться в дифференциалах. Тригонометрические подстановки, которые спасают жизни. Таблица основных интегралов прилагается.",
    tags: ["Матан", "1 курс"],
    author: "Даниил Матанович",
    difficulty: "blue",
  },
  {
    id: 105,
    course: 2,
    title: "Ряды Фурье: Разложение функций",
    date: "05.12.2025",
    previewText:
      "Зачем раскладывать функции в ряд? Условия Дирихле. Четные и нечетные функции. Как это применяется в реальной жизни (спойлер: обработка сигналов).",
    tags: ["Матан", "2 курс"],
    author: "Даниил Матанович",
    difficulty: "red",
  },
  {
    id: 106,
    course: 1,
    title: "Аналитическая геометрия: Кривые второго порядка",
    date: "20.11.2025",
    previewText:
      "Эллипс, гипербола, парабола. Как привести уравнение к каноническому виду и построить график. Инварианты кривых второго порядка.",
    tags: ["Линал", "1 курс"],
    author: "Даниил Матанович",
    difficulty: "none",
  },
];

const ProfilePage = () => {
  return (
    <div className="container mx-auto max-w-screen-xl px-4 md:px-8 py-10 space-y-10">
      <div className="flex flex-col md:flex-row items-start gap-6 md:gap-10">
        <div className="shrink-0">
          <div className="w-32 h-32 md:w-40 md:h-40 bg-secondary/30 rounded-xl overflow-hidden flex items-center justify-center border border-border/50 shadow-sm">
            {MOCK_PROFILE.avatar ? (
              <img
                src={MOCK_PROFILE.avatar}
                alt={MOCK_PROFILE.name}
                className="w-full h-full object-cover"
              />
            ) : (
              <User className="w-16 h-16 text-muted-foreground/40" />
            )}
          </div>
        </div>

        <div className="space-y-4 mt-1">
          <div className="space-y-2">
            <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
              {MOCK_PROFILE.name}
            </h1>

            {MOCK_PROFILE.isVerified && (
              <Badge
                variant="secondary"
                className="bg-emerald-100 text-emerald-700 hover:bg-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400 border-emerald-200 dark:border-emerald-800 px-3 py-1 text-sm font-medium gap-1.5 w-fit flex items-center"
              >
                <ShieldCheck className="w-4 h-4" />
                Проверенный автор
              </Badge>
            )}
          </div>

          <p className="text-lg text-muted-foreground leading-relaxed max-w-2xl">
            {MOCK_PROFILE.bio}
          </p>
        </div>
      </div>

      <div className="border-t border-border/40 pt-8">
        <h2 className="text-2xl font-bold mb-6 hidden md:block">
          Материалы автора
        </h2>
        <MaterialsSection materials={AUTHOR_MATERIALS} />
      </div>
    </div>
  );
};

export default ProfilePage;
