import { Badge, Tabs, TabsList, TabsTrigger } from "@/shared/ui";

import { MODERATION_TABS, ModerationTabValue } from "../model";

type ModerationTabsProps = {
  value: ModerationTabValue;
  counts: Record<ModerationTabValue, number>;
  onValueChange: (value: ModerationTabValue) => void;
};

export const ModerationTabs = ({
  value,
  counts,
  onValueChange,
}: ModerationTabsProps) => (
  <Tabs
    value={value}
    onValueChange={(nextValue) =>
      onValueChange(nextValue as ModerationTabValue)
    }
    className="flex-col gap-4"
  >
    <TabsList className="h-auto w-full flex-wrap justify-start gap-2 bg-transparent p-0">
      {MODERATION_TABS.map((tab) => (
        <TabsTrigger
          key={tab.value}
          value={tab.value}
          className="h-auto flex-none rounded-full border border-border bg-card px-4 py-2 data-[state=active]:bg-muted data-[state=active]:text-foreground data-[state=active]:shadow-none after:hidden"
        >
          {tab.label}
          <Badge variant="secondary">{counts[tab.value]}</Badge>
        </TabsTrigger>
      ))}
    </TabsList>
  </Tabs>
);
