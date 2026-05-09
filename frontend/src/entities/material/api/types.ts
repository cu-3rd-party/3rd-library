import { RealWorldProfile } from "@/entities/user/api";

export type LibraryMaterial = {
  id: string;
  author_id: string;
  author_name?: string | null;
  title: string;
  description: string;
  courses: string[];
  subjects: string[];
  type: string;
  difficulty: string;
  pub_date?: string | null;
};

export type LibraryMaterialFile = {
  id: string;
  name: string;
  size_bytes: number;
  extension: string;
  mime_type?: string | null;
  url?: string | null;
};

export type LibraryPaginatedMaterialsResponse = {
  items: LibraryMaterial[];
  page: number;
  limit: number;
  total: number;
};

export type LibraryMaterialDetailsResponse = {
  id: string;
  author_id: string;
  author_name?: string | null;
  author_image?: string | null;
  title: string;
  description: string;
  courses: string[];
  subjects: string[];
  type: string;
  difficulty: string;
  pub_date?: string | null;
  files: LibraryMaterialFile[];
  published_at?: string | null;
  submitted_at?: string | null;
};

// ! TODO: review slop.
export type RealWorldArticle = {
  slug: string;
  title: string;
  description: string;
  body?: string;
  tagList: string[];
  createdAt: string;
  updatedAt: string;
  favorited: boolean;
  favoritesCount: number;
  author: RealWorldProfile;
};

export type RealWorldArticlesResponse = {
  articles: RealWorldArticle[];
  articlesCount: number;
};

export type RealWorldArticleResponse = {
  article: RealWorldArticle;
};

export type RealWorldAttachment = {
  attachmentId: string;
  fileName: string;
  url?: string | null;
  downloadUrl?: string | null;
  createdAt: string;
  updatedAt: string;
};

export type RealWorldAttachmentsResponse = {
  attachments: RealWorldAttachment[];
};
