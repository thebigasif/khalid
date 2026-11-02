import { getRouteApi } from "@tanstack/react-router";
import { PageHeader } from "../components/PageHeader";
import { AddFile } from "../media/AddFile";
import { Library } from "../media/Library";
import { en } from "../strings/en";

const route = getRouteApi("/signed-in/media");

/** The media library: a file in, and every file already there. */
export function Media() {
  const search = route.useSearch();
  return (
    <>
      <PageHeader title={en.media.title} lede={en.media.lede} />
      <AddFile />
      <Library search={search} />
    </>
  );
}
