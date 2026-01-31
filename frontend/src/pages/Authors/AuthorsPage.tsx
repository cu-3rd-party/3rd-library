import { Search } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

import {Author} from "@/common/molecules/Authors/AuthorCard.tsx";
import {AuthorsGrid} from "@/common/organisms";
import {Input} from "@/components/ui/input.tsx";


const MOCK_AUTHORS: Author[] = [
  { id: 1, name: "Матан Матанович" },
  { id: 2, name: "Линал Линалович" },
  { id: 3, name: "Это Дима Трушин" },
  { id: 4, name: "Это магадан" },
  { id: 5, name: "Горо горо)" },
  { id: 6, name: "Матан Матанович" },
  { id: 7, name: "Матан Матанович" },
  { id: 8, name: "Матан Матанович" },
];

export const AuthorsPage = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");

  const filteredAuthors = MOCK_AUTHORS.filter((author) =>
    author.name.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  const handleAuthorClick = (id: number) => {
    navigate(`/authors/${id}`);
  };

  return (
    <div className="container w-full md:w-5/6 mx-auto max-w-screen-2xl py-4 md:py-6 px-4 md:px-0 space-y-6 md:space-y-8">
      <div className="flex justify-end">
        <div className="relative w-full md:w-72">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9 bg-secondary/30 border-border/50 focus-visible:ring-orange-500 rounded-lg h-10 md:h-10"
          />
        </div>
      </div>

      <AuthorsGrid
        authors={filteredAuthors}
        onAuthorClick={handleAuthorClick}
      />
    </div>
  );
};