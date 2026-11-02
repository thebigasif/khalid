import { useQuery } from "@tanstack/react-query";
import { isData } from "../../api/call";
import { createApi } from "../../api/client";
import { isProblem } from "../../api/posts";
import type { Reply } from "../../api/redirects";
import type { components } from "../../api/schema";

/** Whether suggestions are on, and who writes them. */
export type AiState = components["schemas"]["AiState"];
/** What a suggestion is written for and from. */
export type SuggestionRequest = components["schemas"]["SuggestionRequest"];
/** The options to pick from, fitting ones first. */
export type Offered = components["schemas"]["Offered"];
/** One option, with its length checked. */
export type Suggestion = components["schemas"]["Suggestion"];

const api = createApi();

/** The data, or the problem the API sent; status 0 when the blog was unreachable. */
function reply<T>(data: T | undefined, error: unknown, response: Response): Reply<T> {
  if (response.ok && data !== undefined) {
    return { data };
  }
  return { problem: isProblem(error) ? error : null, status: response.status };
}

const UNREACHABLE = { problem: null, status: 0 };

/** Reads whether the owner turned suggestions on and the server holds its key. */
export async function readAi(): Promise<Reply<AiState>> {
  try {
    const { data, error, response } = await api.GET("/api/admin/v1/ai/");
    return reply(data, error, response);
  } catch {
    return UNREACHABLE;
  }
}

/** Asks the owner's AI service for options. Nothing in a post changes until one is picked. */
export async function suggest(request: SuggestionRequest): Promise<Reply<Offered>> {
  try {
    const route = "/api/admin/v1/ai/suggestions/";
    const { data, error, response } = await api.POST(route, { body: request });
    return reply(data, error, response);
  } catch {
    return UNREACHABLE;
  }
}

/**
 * Whether the AI buttons show: only when the owner turned suggestions on
 * and the server can ask. Read once, since it changes only on restart.
 */
export function useAiReady(): boolean {
  const state = useQuery({
    queryKey: ["ai"],
    queryFn: readAi,
    staleTime: Number.POSITIVE_INFINITY,
  });
  return isData(state.data) && state.data.data.ready === true;
}
