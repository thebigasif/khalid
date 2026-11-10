import { calendarDay } from "../strings/dates";
import { en } from "../strings/en";
import { MAX_NOTE } from "../strings/realUpdateWords";

/** The switch as the writer left it: on or off, and the note as typed. */
export type Marking = { on: boolean; note: string };

/** The last real update readers see: its day, such as `2026-10-31`, and its note. */
export type Dated = { day: string | null; note: string | null };

/** The note to send, trimmed, or `null` when it is empty or too long for the API. */
export function noteToSend(marking: Marking): string | null {
  const note = marking.note.trim();
  if (!marking.on || note === "" || [...note].length > MAX_NOTE) {
    return null;
  }
  return note;
}

type Props = { dated: Dated; marking: Marking; onMarking: (marking: Marking) => void };

/** The honest update switch: a live post's date moves only when the writer says what changed. */
export function RealUpdate({ dated, marking, onMarking }: Props) {
  const words = en.realUpdate;
  const length = [...marking.note.trim()].length;
  let problem: string | null = null;
  if (marking.on && length === 0) {
    problem = words.empty;
  } else if (length > MAX_NOTE) {
    problem = words.tooLong;
  }
  return (
    <fieldset>
      <legend>{words.title}</legend>
      <span className="hint">{words.hint}</span>
      <p className="muted">
        {dated.day === null ? words.none : words.last(calendarDay(dated.day), dated.note ?? "")}
      </p>
      <label className="choice">
        <input
          type="checkbox"
          checked={marking.on}
          onChange={(event) => onMarking({ ...marking, on: event.target.checked })}
        />
        <span>{words.on}</span>
      </label>
      {marking.on ? (
        <div className="field">
          <label htmlFor="real-update-note">{words.note}</label>
          <span className="hint" id="real-update-note-hint">
            {words.noteHint(length)}
          </span>
          <input
            id="real-update-note"
            aria-describedby="real-update-note-hint"
            aria-invalid={problem !== null}
            value={marking.note}
            onChange={(event) => onMarking({ ...marking, note: event.target.value })}
          />
          {problem === null ? null : <p className="careful">{problem}</p>}
        </div>
      ) : null}
    </fieldset>
  );
}
