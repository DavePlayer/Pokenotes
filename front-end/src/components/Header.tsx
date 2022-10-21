import { Link } from "react-router-dom";

interface IRoute {
  pathName: String;
  pathRoute: String;
}

const routes: Array<IRoute> = [
  { pathName: "Pokenotes", pathRoute: "/" },
  { pathName: "Pokemons", pathRoute: "/pokemons" },
  { pathName: "Shiny Tactics", pathRoute: "/tactics" },
  { pathName: "Moves", pathRoute: "/moves" },
  { pathName: "Abilities", pathRoute: "/abilities" },
  { pathName: "Pokemon Types", pathRoute: "/types" },
  { pathName: "Games", pathRoute: "/games" },
  { pathName: "Pokedexes", pathRoute: "/pokedexes" },
  { pathName: "Locations", pathRoute: "/locations" },
  { pathName: "Catch Techciques", pathRoute: "/techniques" },
];

export const Header = () => {
  return (
    <header className="w-full min-h-[2rem] p-[1rem] bg-default-darkest">
      <ul className="text-xs w-3/4 flex justify-between">
        {routes.map((route) => (
          <>
            <Link className="header-hover" to={`${route.pathRoute}`}>
              <li>{route.pathName}</li>
            </Link>
          </>
        ))}
      </ul>
    </header>
  );
};
