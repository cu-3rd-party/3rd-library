import { Search } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { AuthorsGrid } from "@/common/organisms";
import { Input } from "@/components/ui/input";
import { MOCK_AUTHORS } from "@/mocks/mockData";

const AuthorsPage = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");

  const filteredAuthors = MOCK_AUTHORS.filter((author) =>
    author.name.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  const handleAuthorClick = (id: string) => {
    navigate(`/authors/${id}`);
  };

  return (
    <div className="w-full md:w-11/12 xl:w-5/6 mx-auto max-w-screen-2xl py-4 px-4 space-y-6 lg:space-y-8">
      <div className="flex justify-end">
        <div className="relative w-full lg:w-72">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9 border-border rounded-lg h-10 lg:h-10"
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

export default AuthorsPage;
