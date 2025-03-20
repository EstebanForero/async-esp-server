import { createSignal } from 'solid-js';
import './App.css';
import Home from './routes/Home';
import Settings from './routes/Settings';
import Navbar from './components/Navbar';
import Values from './routes/Values';
import History from './routes/History';

export type Routes = "Home" | "Settings" | "Values" | "History";

export const ROUTES: Routes[] = ["Home", "Settings", "Values", "History"]

function App() {
  const [currentRoute, setCurrentRoute] = createSignal<Routes>("Home");

  return (
    <>
      <Navbar setCurrentRoute={setCurrentRoute} />
      {currentRoute() === "History" && <History />}
      {currentRoute() === "Values" && <Values />}
      {currentRoute() === "Home" && <Home />}
      {currentRoute() === "Settings" && <Settings />}
    </>
  );
}

export default App;
