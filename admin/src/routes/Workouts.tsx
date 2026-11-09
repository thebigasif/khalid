import { useState } from "react";
import { PageHeader } from "../components/PageHeader";
import { CornerstoneWorkout } from "../seo/CornerstoneWorkout";
import { OrphanWorkout } from "../seo/OrphanWorkout";
import { seoToolWords } from "../strings/seoToolWords";

type Workout = keyof typeof seoToolWords.workouts.tabs;

const WORKOUTS: Workout[] = ["orphans", "cornerstone"];

/** Yoast's SEO workouts: orphaned posts and cornerstone links, one workout at a time. */
export function Workouts() {
  const words = seoToolWords.workouts;
  const [workout, setWorkout] = useState<Workout>("orphans");
  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      <fieldset className="switch spaced" aria-label={words.pick}>
        {WORKOUTS.map((each) => (
          <button
            key={each}
            type="button"
            aria-pressed={each === workout}
            onClick={() => setWorkout(each)}
          >
            {words.tabs[each]}
          </button>
        ))}
      </fieldset>
      <section className="focus wide" aria-labelledby="workout-title">
        <h2 id="workout-title">{words.tabs[workout]}</h2>
        {workout === "orphans" ? <OrphanWorkout /> : <CornerstoneWorkout />}
      </section>
    </>
  );
}
