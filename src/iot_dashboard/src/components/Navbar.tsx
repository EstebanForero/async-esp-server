
import { Setter } from 'solid-js';

type Routes = "Home" | "Settings";
const routes: Routes[] = ["Home", "Settings"];

function Navbar(props: { setCurrentRoute: Setter<Routes> }) {
  return (
    <nav class="navbar">
      {routes.map((route) => (
        <button
          class="nav-link"
          onClick={() => props.setCurrentRoute(route)}
        >
          {route}
        </button>
      ))}
    </nav>
  );
}

export default Navbar;
