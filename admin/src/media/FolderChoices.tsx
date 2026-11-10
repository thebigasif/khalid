import { useQuery } from "@tanstack/react-query";
import { listFolders } from "../api/media";
import { isData } from "../api/redirects";

/** The id a folder field names in its `list` to be offered the library's folders. */
export const FOLDER_CHOICES = "media-folder-choices";

/**
 * The library's folders, offered as the owner types a folder. A new folder
 * can still be typed; it is made with the first file filed in it.
 */
export function FolderChoices() {
  const reply = useQuery({ queryKey: ["media", "folders"], queryFn: listFolders });
  const folders = isData(reply.data) ? reply.data.data : [];
  return (
    <datalist id={FOLDER_CHOICES}>
      {folders.map((folder) => (
        <option key={folder} value={folder} />
      ))}
    </datalist>
  );
}
