import { Icon } from "./Icon";

/**
 * A short message raised at the foot of the screen, such as "Saved", that
 * fades after a few seconds. The live region itself stays put, so screen
 * readers hear each new message; each message starts its own fade.
 */
export function Toast({ message }: { message: string | null }) {
  return (
    <p className="toast-region" role="status">
      {message === null || message === "" ? null : (
        <span key={message} className="toast">
          <Icon name="check" />
          {message}
        </span>
      )}
    </p>
  );
}
