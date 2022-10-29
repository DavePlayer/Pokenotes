import { ElementalList } from "../Elementals/ElementalList";
import { ElementalPages } from "../Elementals/ElementsPages";
import { TopFilters } from "../Elementals/TopFilters";

export const Pokemons = () => {
  return (
    <>
      <TopFilters />
      <ElementalPages>
        <ElementalList tableName="pokemons" />
      </ElementalPages>
    </>
  );
};
