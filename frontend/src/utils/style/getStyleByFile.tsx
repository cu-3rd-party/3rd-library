import { File, FileType, FileChartPie, FileCode, FileText } from "lucide-react";
import { ReactNode } from "react";

export const getFileIconStyles = (fileName: string): string => {
  const name = fileName.toLowerCase();
  if (name.endsWith(".pdf")) return "bg-secondary text-secondary-foreground";
  if (name.endsWith(".doc") || name.endsWith(".docx"))
    return "bg-blue-badge text-blue-badge-foreground";
  if (name.endsWith(".ppt") || name.endsWith(".pptx"))
    return "bg-destructive/10 text-destructive";
  if (name.endsWith(".ipynb"))
    return "bg-light-blue-badge text-light-blue-badge-foreground";
  return "bg-muted text-muted-foreground";
};

export const getFileIcon = (fileName: string): ReactNode => {
  const name = fileName.toLowerCase();
  if (name.endsWith(".pdf")) return <FileText className="h-6 w-6" />;
  if (name.endsWith(".doc") || name.endsWith(".docx"))
    return <FileType className="h-6 w-6" />;
  if (name.endsWith(".ppt") || name.endsWith(".pptx"))
    return <FileChartPie className="h-6 w-6" />;
  if (name.endsWith(".ipynb")) return <FileCode className="h-6 w-6" />;
  return <File className="h-6 w-6" />;
};
