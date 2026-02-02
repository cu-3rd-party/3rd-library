import { FileCode, FileText } from "lucide-react";

export const getFileIconStyles = (fileName: string) => {
  const name = fileName.toLowerCase();
  if (name.endsWith(".pdf")) return "bg-red-500/10 text-red-500";
  if (name.endsWith(".doc") || name.endsWith(".docx"))
    return "bg-blue-500/10 text-blue-500";
  if (name.endsWith(".ppt") || name.endsWith(".pptx"))
    return "bg-orange-500/10 text-orange-500";
  if (name.endsWith(".ipynb")) return "bg-yellow-500/10 text-yellow-600";
  return "bg-muted text-muted-foreground";
};

export const getFileIcon = (fileName: string) => {
  if (fileName.toLowerCase().endsWith(".ipynb")) {
    return <FileCode className="h-6 w-6" />;
  }
  return <FileText className="h-6 w-6" />;
};