FSD - Feature-Sliced Design

src/
├── app/              # инициализация (бывший main.tsx + провайдеры)
│   ├── providers/    # BrowserRouter, QueryClient, StrictMode
│   ├── styles/       # index.css
│   └── index.tsx
├── pages/            # композиция фич, без бизнес-логики
│   ├── materials-list/
│   ├── material-details/
│   ├── material-upload/
│   ├── moderation/
│   ├── authors/
│   ├── authorization/
│   └── about/
├── widgets/          # большие самостоятельные блоки (ранее - organisms)
│   ├── materials-section/
│   ├── header/
│   └── navbar/
├── features/         # пользовательские сценарии
│   ├── filter-materials/       # MaterialFilter + filterSortStore (filter-часть)
│   ├── sort-materials/         # MaterialSort + sortState
│   ├── search-materials/       # из MaterialsSection (searchQuery)
│   ├── upload-material/        # UploadMaterialForm + materialSubmissionStore
│   ├── moderate-material/
│   ├── auth-login/
│   └── toggle-theme/           # useTheme
├── entities/         # бизнес-сущности: модель + UI отображения
│   ├── material/
│   │   ├── model/    # types (бывший models/material.ts), api hooks
│   │   ├── ui/       # MaterialCard, MaterialBadge
│   │   └── lib/      # mapArticleToMaterial, getCourseName
│   ├── user/
│   │   ├── model/    # currentUser, types
│   │   └── lib/      # canAccessModeration
│   ├── author/
│   └── submission/
└── shared/           # переиспользуемое без бизнес-знания
    ├── ui/           # shadcn
    ├── api/          # fetchJson, resolveApiUrl, ApiRequestError (бывший lib/api.ts)
    ├── config/       # routePrefixes, env
    ├── lib/          # cn, утилиты (utils/, lib/utils.ts)
    └── constants/    # difficulty, materialType, sort

удалять mock service worker