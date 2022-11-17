import { ElementalList } from "../Elementals/ElementalList";
import { ElementalPages } from "../Elementals/ElementsPages";
import { TopFilters } from "../Elementals/TopFilters";
import { storeState } from "../../store/store";
import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { elementalsActions, PageManagerActions } from "./../../store/actionTypes"

export const Pokemons = () => {
  const dispatch = useDispatch()
  const pageManager = useSelector((store: storeState) => store.pageManager)
  useEffect(() => {
    const elementsinDatabease = 1200;
    dispatch({ type: PageManagerActions.SETLENGTH, payload: elementsinDatabease })
    let ar = Array.apply(null, Array(elementsinDatabease)).map((el, i) => { return { name: `twoja mama ${i}` } })
    console.log((pageManager.selectedPage - 1) * 40, (pageManager.selectedPage * 40) > elementsinDatabease ? elementsinDatabease : pageManager.selectedPage * 40 + 1)
    ar = ar.slice((pageManager.selectedPage - 1) * 40, (pageManager.selectedPage * 40) > elementsinDatabease ? elementsinDatabease : pageManager.selectedPage * 40 + 1)
    dispatch({ type: elementalsActions.SET, payload: ar })
  }, [pageManager.selectedPage])
  const list = useSelector((store: storeState) => store.elementalList);

  return (
    <>
      <TopFilters />
      <ElementalPages>
        <ElementalList list={list} />
      </ElementalPages>
    </>
  );
};
