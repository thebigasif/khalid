import { useQuery } from "@tanstack/react-query";
import { createApi } from "./client";
import type { components } from "./schema";

/** A category as the Admin API sends it: its slug and its name in each language. */
export type Category = components["schemas"]["AdminCategory"];

const api = createApi();

/** Every category of the site, or none when the list cannot be read. */
export async function listCategories(): Promise<Category[]> {
  try {
    const { data } = await api.GET("/api/admin/v1/categories/");
    return data?.categories ?? [];
  } catch {
    return [];
  }
}

/** The site's categories, kept by TanStack Query; empty until they arrive. */
export function useCategories(): Category[] {
  return useQuery({ queryKey: ["categories"], queryFn: listCategories }).data ?? [];
}

/** The name readers see for `category`: its English name, or its first, or its slug. */
export function categoryName(category: Category): string {
  return category.names.en ?? Object.values(category.names)[0] ?? category.slug;
}
