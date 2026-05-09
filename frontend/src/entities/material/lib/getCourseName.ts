import { Course } from "../model";

export const getCourseName = (courses: Course[]): string => {
  const sortedCourses = [...courses].sort();

  if (sortedCourses.length === 0) return "Курс не указан";
  if (sortedCourses.length === 1) return `${sortedCourses[0]} курс`;

  return `${sortedCourses[0]}-${sortedCourses[sortedCourses.length - 1]} курсы`;
};
