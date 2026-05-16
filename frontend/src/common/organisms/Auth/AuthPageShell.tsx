import type { ReactNode } from "react";

import { AuthThemeToggleButton } from "@/common/atoms/Auth";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export type AuthPageShellProps = {
  title: string;
  description: string;
  headerRight?: ReactNode;
  children: ReactNode;
};

export const AuthPageShell = ({
  title,
  description,
  headerRight,
  children,
}: AuthPageShellProps) => (
  <div className="relative min-h-screen overflow-hidden bg-background px-4 py-10">
    <div className="pointer-events-none absolute inset-0">
      <div className="absolute -left-24 top-0 h-72 w-72 rounded-full bg-primary/20 blur-3xl" />
      <div className="absolute -right-24 bottom-0 h-72 w-72 rounded-full bg-secondary/30 blur-3xl" />
    </div>

    <AuthThemeToggleButton className="absolute right-8 top-8 z-20 md:right-10" />

    <div className="relative mx-auto flex min-h-[calc(100vh-5rem)] w-full max-w-md items-center">
      <Card className="w-full border-border bg-card/95 backdrop-blur">
        <CardHeader className="space-y-2">
          <div className="flex items-start justify-between gap-3">
            <CardTitle className="text-2xl font-bold tracking-tight">
              {title}
            </CardTitle>
            {headerRight ? <div className="shrink-0">{headerRight}</div> : null}
          </div>
          <p className="text-sm text-muted-foreground">{description}</p>
        </CardHeader>
        <CardContent className="space-y-5">{children}</CardContent>
      </Card>
    </div>
  </div>
);
