import { Link } from "@tanstack/react-router";
import { Icon } from "../components/Icon";
import { dashboardWords as words } from "../strings/dashboardWords";

/** The things a writer starts most, one click each. */
export function QuickActions() {
  return (
    <section aria-labelledby="quick-title">
      <h2 id="quick-title">{words.quick}</h2>
      <ul className="quick">
        <li>
          <Link to="/new">
            <Icon name="plus" />
            {words.quickWrite}
          </Link>
        </li>
        <li>
          <Link to="/media">
            <Icon name="media" />
            {words.quickMedia}
          </Link>
        </li>
        <li>
          <Link to="/comments">
            <Icon name="comments" />
            {words.quickComments}
          </Link>
        </li>
        <li>
          <Link to="/look">
            <Icon name="look" />
            {words.quickLook}
          </Link>
        </li>
      </ul>
    </section>
  );
}
