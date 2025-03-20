import { createSignal } from 'solid-js';
import './App.css';
import Home from './routes/Home';
import Settings from './routes/Settings';
import Navbar from './components/Navbar';

type Routes = "Home" | "Settings";

function App() {
  const [currentRoute, setCurrentRoute] = createSignal<Routes>("Home");

  return (
    <>
      <Navbar setCurrentRoute={setCurrentRoute} />
      {currentRoute() === "Home" && <Home />}
      {currentRoute() === "Settings" && <Settings />}
    </>
  );
}

export default App;
