import { ArrowDownWideNarrow, ArrowUpWideNarrow, BookOpen, Calendar, CaseSensitive } from "lucide-react";
import { ReactNode } from "react";

import { SortOrderType, SortType } from "@/models";

export const SORT_TYPE_CONFIG: Record<
  SortType,
  { label: string, icon: ReactNode }
> = {
  date: {
    label: "По дате",
    icon: <Calendar className="size-5" />,
  },
  title: {
    label: "По названию",
    icon: <CaseSensitive className="size-5" />,
  },
  subject: {
    label: "По предмету",
    icon: <BookOpen className="size-5" />,
  },
};

export const SORT_ORDER_CONFIG: Record<
  SortOrderType,
  { label: string, icon: ReactNode }
> = {
  decreasing: {
    label: "По убыванию",
    icon: <ArrowDownWideNarrow className="size-5" />,
  },
  increasing: {
    label: "По возрастанию",
    icon: <ArrowUpWideNarrow className="size-5" />,
  },
};