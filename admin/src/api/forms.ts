// What visitors sent through form blocks, read from the Admin API and
// deleted on request (backlog row 21, gap audit 4 row 11, KH-GRO-1).
import { createApi } from "./client";
import { isProblem } from "./posts";
import type { Failure, Reply } from "./redirects";
import type { components } from "./schema";

/** One form and how many answers it has. */
export type FormSummary = components["schemas"]["FormSummary"];
/** One answer a visitor sent. */
export type FormAnswer = components["schemas"]["FormAnswer"];
/** One page of a form's answers, and where the page sits. */
export type Answers = components["schemas"]["AdminAnswers"];

/** How many answers one page holds. */
export const ANSWERS_PER_PAGE = 10;

const api = createApi();

/** The reply to a call that failed: the problem the API sent, if any. */
function failed(error: unknown, response: Response): Failure {
  return { problem: isProblem(error) ? error : null, status: response.status };
}

/** The reply when the blog cannot be reached at all. */
const UNREACHABLE: Failure = { problem: null, status: 0 };

/** Lists each form visitors answered, the one answered last first. */
export async function listForms(): Promise<Reply<FormSummary[]>> {
  try {
    const { data, error, response } = await api.GET("/api/admin/v1/forms/");
    return data === undefined ? failed(error, response) : { data: data.forms };
  } catch {
    return UNREACHABLE;
  }
}

/** Lists page `page` of the answers to form `id`, newest first. */
export async function listAnswers(id: string, page: number): Promise<Reply<Answers>> {
  try {
    const { data, error, response } = await api.GET("/api/admin/v1/forms/{id}/answers/", {
      params: { path: { id }, query: { page, limit: ANSWERS_PER_PAGE } },
    });
    return data === undefined ? failed(error, response) : { data };
  } catch {
    return UNREACHABLE;
  }
}

/** Deletes answer `answer` to form `id` for good. */
export async function deleteAnswer(id: string, answer: number): Promise<Reply<null>> {
  try {
    const { error, response } = await api.DELETE("/api/admin/v1/forms/{id}/answers/{answer}/", {
      params: { path: { id, answer } },
    });
    return response.ok ? { data: null } : failed(error, response);
  } catch {
    return UNREACHABLE;
  }
}
