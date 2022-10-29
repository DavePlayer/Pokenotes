import { Link, useLocation } from "react-router-dom";

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
  const location = useLocation();
  console.log(location);

  return (
    <header className="w-full min-h-[2rem] p-[0.75rem] bg-default-darkest">
      <ul className="text-xs w-3/4 flex justify-between">
        {routes.map((route, i) => (
          <Link key={i} className="header-hover" to={`${route.pathRoute}`}>
            <li
              className={location.pathname.replace("/", "") == route.pathRoute.replace("/", "") ? "route-selected" : ""}
            >
              {route.pathName}
            </li>
          </Link>
        ))}
      </ul>
    </header>
  );
};
