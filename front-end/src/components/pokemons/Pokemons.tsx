import { ElementalList } from "../Elementals/ElementalList";
import { ElementalPages } from "../Elementals/ElementsPages";
import { TopFilters } from "../Elementals/TopFilters";
import { storeState } from "../../store/store";
import { useEffect } from "react";
import { useDispatch } from "react-redux";
import { elementalsActions } from "./../../store/actionTypes"

export const Pokemons = () => {
  const dispatch = useDispatch()
  useEffect(() => {
    const ar = Array.apply(null, Array(100)).map((el, i) => { return { name: `twoja mama ${i}` } })
    console.log(ar)
    dispatch({ type: elementalsActions.SET, payload: ar })
  }, [])

  return (
    <>
      <TopFilters />
      <ElementalPages>
        <ElementalList />
      </ElementalPages>
    </>
  );
};
