import { Route, Routes } from "react-router-dom";
import { Home } from "./components/Home";
import { Header } from "./components/Header";
import { ErrorPath } from "./components/ErrorPath";
import { Pokemons } from "./components/pokemons/Pokemons";

function App() {
  return (
    <>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/pokemons" element={<Pokemons />} />
        <Route path="*" element={<ErrorPath details={"Page not found"} status={404} />} />
      </Routes>
    </>
  );
}

export default App;
