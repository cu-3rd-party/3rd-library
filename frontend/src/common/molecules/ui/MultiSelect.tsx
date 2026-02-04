  import { Check, ChevronDown } from "lucide-react";
  import { ReactNode } from "react";

  import { Button } from "@/components/ui/button";
  import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
  } from "@/components/ui/command";
  import {
    Popover,
    PopoverContent,
    PopoverTrigger,
  } from "@/components/ui/popover";
  import { cn } from "@/lib/utils";

  type MultiSelectProps<T> = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    selectedItems: T[];
    onToggle: (item: T) => void;
    items: T[];
    placeholder: ReactNode;
    emptyText: string;
    searchPlaceholder?: string;
    renderBadge: (item: T) => ReactNode;
    renderItem: (item: T) => ReactNode;
    getItemKey: (item: T) => string;
    getItemValue: (item: T) => string;
    isSelected: (item: T) => boolean;
  };

  export function MultiSelect<T>({
    open,
    onOpenChange,
    selectedItems,
    onToggle,
    items,
    placeholder,
    emptyText,
    searchPlaceholder,
    renderBadge,
    renderItem,
    getItemKey,
    getItemValue,
    isSelected,
  }: MultiSelectProps<T>) {
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
                selectedItems.map((item) => (
                  <div key={getItemKey(item)}>{renderBadge(item)}</div>
                ))
              ) : (
                <span className="text-muted-foreground font-normal text-base">
                  {placeholder}
                </span>
              )}
            </div>
            <ChevronDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-(--radix-popover-trigger-width) p-0" align="start">
          <Command>
            {searchPlaceholder && <CommandInput placeholder={searchPlaceholder} />}
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
          </Command>
        </PopoverContent>
      </Popover>
    );
  }