import { create } from 'zustand';
import { persist } from 'zustand/middleware';

import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from '@/constants';
import { FilterState, SortState, defaultFilterState, defaultSortState } from "@/models"

type FilterSortStore = {
  filterState: FilterState;
  sortState: SortState;
  setFilterState: <K extends keyof FilterState>(key: K, value: FilterState[K]) => void;
  setSortState: <K extends keyof SortState>(key: K, value: SortState[K]) => void;
  toggleFilter: (key: keyof FilterState, newFilter: string ) => void;
  resetFilter: (key: keyof FilterState) => void;
  resetFilters: () => void;
  resetAll: () => void;
}

// !IMPORTANT may think of non-persistent
export const useFilterSortStore = create<FilterSortStore>()(
  persist(
    (set) => ({
      filterState: defaultFilterState,
      sortState: defaultSortState,
    
      setFilterState: (key, value) => 
        set((state) => ({
          filterState: {...state.filterState, [key]: value},
        })),
      
      setSortState: (key, value) => 
        set((state) => ({
          sortState: {...state.sortState, [key]: value},
        })),

      toggleFilter: (key, newFilter) => 
        set((state) => {
          const prev = state.filterState[key] as string[];
          const newValues = prev.includes(newFilter) ?
            prev.filter(val => val !== newFilter) :
            [...prev, newFilter];
            
          const orderMap: Record<keyof FilterState, readonly string[]> = {
            courses: COURSES,
            subjects: SUBJECTS,
            difficulties: DIFFICULTIES,
            types: MATERIAL_TYPES
          }
          
          const sorted = newValues.sort((a, b) => orderMap[key].indexOf(a) - orderMap[key].indexOf(b));

          return {
            filterState: {
              ...state.filterState,
              [key]: sorted
            }
          }
        }),

      resetFilter: (key) => 
        set((state) => ({
          filterState: {...state.filterState, [key]: []}
        })),
        
      resetFilters: () => set({filterState: defaultFilterState}),

      resetAll: () => 
        set({
          filterState: defaultFilterState,
          sortState: defaultSortState
        }),
      
    }),
    {
      name: "material-filter-sort-store",
    }
  )
);