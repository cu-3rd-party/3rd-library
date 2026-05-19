import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

import { User } from "@/entities/user/model";

import { fetchAuthors } from "../api";

export const useAuthors = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");
  const [authors, setAuthors] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
    const ctrl = new AbortController();

    fetchAuthors(ctrl.signal)
      .then((fetched) => {
        if (ctrl.signal.aborted) return;
        setAuthors(fetched);
      })
      .catch((error: unknown) => {
        if (ctrl.signal.aborted) return;
        console.error(error);
        setIsError(true);
      })
      .finally(() => {
        if (!ctrl.signal.aborted) setIsLoading(false);
      });

    return () => ctrl.abort();
  }, []);

  const filteredAuthors = useMemo(
    () => authors.filter((a) => a.name.toLowerCase().includes(searchQuery.toLowerCase())),
    [authors, searchQuery],
  );

  const handleAuthorClick = (id: string) => {
    navigate(`/authors/${encodeURIComponent(id)}`);
  };

  return { searchQuery, setSearchQuery, filteredAuthors, isLoading, isError, handleAuthorClick };
};
