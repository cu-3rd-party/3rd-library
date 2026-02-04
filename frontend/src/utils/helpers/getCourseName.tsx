import { Course } from "@/models";

export const getCourseName = (courses: Course[]) => {
  const sortedCourses = courses.sort();
  if (sortedCourses.length == 1) return `${sortedCourses[0]} курс`
  else return `${sortedCourses[0]}-${sortedCourses[sortedCourses.length - 1]} курсы`
}