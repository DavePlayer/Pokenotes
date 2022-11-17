import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useSearchParams } from "react-router-dom";
import { storeState } from "./../../store/store";
import { PageManagerActions } from "./../../store/actionTypes"
import { FirstButton } from "./buttons/first";
import { PrevButton } from "./buttons/prev";
import { NextButton } from "./buttons/next";
import { LastButton } from "./buttons/last";
import { IndexedButton } from "./buttons";

export const ElementalPages: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [searchParams, setSearchParams] = useSearchParams();
  const pageManager = useSelector((store: storeState) => store.pageManager)
  const dispatch = useDispatch()

  const pageByIndex = (index: number) => {
    dispatch({ type: PageManagerActions.INDEX, payload: index })
  }
  useEffect(() => {
    const index = parseInt(searchParams.get("p")!)
    console.log("page updated: ", index);
    pageByIndex(index || 1)
  }, [searchParams]);


  return (
    <>
      <section className="flex center justify-center">
        <FirstButton />
        <PrevButton />
        <IndexedButton />
        <NextButton />
        <LastButton />
      </section>
      <section>{children}</section>
    </>
  );
};
