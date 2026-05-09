import { Check, ChevronDown } from "lucide-react";
import { ReactNode } from "react";

import { cn } from "@/shared/lib";

import {
  Badge,
  Button,
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "./kit";

type MultiSelectProps<T> = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  selectedItems: T[];
  onToggle: (item: T) => void;
  items: readonly T[];
  placeholder: ReactNode;
  emptyText: string;
  searchPlaceholder?: string;
  onReset?: () => void;
  renderBadge: (item: T) => ReactNode;
  renderItem: (item: T) => ReactNode;
  getItemKey: (item: T) => string;
  getItemValue: (item: T) => string;
  isSelected: (item: T) => boolean;
};

export const MultiSelect = <T,>({
  open,
  onOpenChange,
  selectedItems,
  onToggle,
  items,
  placeholder,
  emptyText,
  searchPlaceholder,
  onReset,
  renderBadge,
  renderItem,
  getItemKey,
  getItemValue,
  isSelected,
}: MultiSelectProps<T>) => {
  return (
    <Popover open={open} onOpenChange={onOpenChange}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between h-auto min-h-12 px-3 py-2"
        >
          <div className="flex flex-wrap gap-1 items-center">
            {selectedItems.length > 0 ? (
              <>
                {selectedItems.slice(0, 2).map((item) => (
                  <div key={getItemKey(item)}>{renderBadge(item)}</div>
                ))}
                {selectedItems.length > 2 && (
                  <Badge variant="secondary">+{selectedItems.length - 2}</Badge>
                )}
              </>
            ) : (
              <span className="text-muted-foreground font-normal text-base">
                {placeholder}
              </span>
            )}
          </div>
          <ChevronDown className="h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent
        className="w-(--radix-popover-trigger-width) p-0"
        align="start"
      >
        <Command>
          {searchPlaceholder && (
            <CommandInput placeholder={searchPlaceholder} />
          )}
          <CommandList>
            <CommandEmpty>{emptyText}</CommandEmpty>
            <CommandGroup>
              {items.map((item) => (
                <CommandItem
                  key={getItemKey(item)}
                  value={getItemValue(item)}
                  onSelect={() => onToggle(item)}
                >
                  <Check
                    className={cn(
                      "mr-2 h-4 w-4",
                      isSelected(item) ? "opacity-100" : "opacity-0",
                    )}
                  />
                  {renderItem(item)}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
          {onReset && selectedItems.length > 0 && (
            <>
              <CommandSeparator />
              <div className="p-1">
                <Button
                  variant="ghost"
                  size="lg"
                  className="w-full h-8 text-sm justify-center bg-input hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-none"
                  onClick={onReset}
                >
                  Сбросить фильтры
                </Button>
              </div>
            </>
          )}
        </Command>
      </PopoverContent>
    </Popover>
  );
};
