
import { Setter } from 'solid-js';
import { ROUTES, Routes } from '../App';

function Navbar(props: { setCurrentRoute: Setter<Routes> }) {
  return (
    <nav class="navbar">
      {ROUTES.map((route) => (
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
