export const isPathActive = (path: string) => {
  if (path === "/" && location.pathname !== "/") return false;
  return location.pathname === path;
};
