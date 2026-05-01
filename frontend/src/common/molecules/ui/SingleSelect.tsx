import { ChevronDown, Check } from "lucide-react";
import { ReactNode } from "react";

import { Button } from "@/components/ui/button";
import {
  Command,
  CommandGroup,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { cn } from "@/lib/utils";

type SingleSelectProps<T> = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  selectedItem: T | null;
  onSelect: (item: T) => void;
  items: readonly T[];
  placeholder?: string;
  renderItem: (item: T) => ReactNode;
  getItemKey: (item: T) => string;
  getItemValue: (item: T) => string;
  getItemIcon?: (item: T) => ReactNode;
};

export const SingleSelect = <T,>({
  open,
  onOpenChange,
  selectedItem,
  onSelect,
  items,
  placeholder,
  renderItem,
  getItemKey,
  getItemValue,
  getItemIcon,
}: SingleSelectProps<T>) => {
  return (
    <Popover open={open} onOpenChange={onOpenChange}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between min-h-12 px-3 py-2"
        >
          <span
            className={cn(
              "flex items-center justify-center gap-2",
              selectedItem
                ? "font-medium text-base"
                : "font-normal text-muted-foreground text-base",
            )}
          >
            {selectedItem ? (
              <>
                {getItemIcon && (
                  <span className="mr-2 h-4 w-4 flex items-center justify-center">
                    {getItemIcon(selectedItem)}
                  </span>
                )}
                {renderItem(selectedItem)}
              </>
            ) : (
              placeholder
            )}
          </span>
          <ChevronDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent
        className="w-(--radix-popover-trigger-width) p-0"
        align="start"
      >
        <Command>
          <CommandList>
            <CommandGroup>
              {items.map((item) => (
                <CommandItem
                  key={getItemKey(item)}
                  value={getItemValue(item)}
                  onSelect={() => {
                    onSelect(item);
                    onOpenChange(false);
                  }}
                >
                  {getItemIcon && (
                    <span className="mr-2 h-4 w-4 flex items-center justify-center">
                      {getItemIcon(item)}
                    </span>
                  )}
                  {renderItem(item)}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};
