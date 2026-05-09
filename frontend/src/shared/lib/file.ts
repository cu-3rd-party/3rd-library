export const getFileExtension = (fileName: string) => {
  const fileNameParts = fileName.split(".");
  return fileNameParts.length > 1
    ? fileNameParts[fileNameParts.length - 1].toLowerCase()
    : "";
};
