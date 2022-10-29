import React from "react";
import { useSelector } from "react-redux";
import { storeState } from "./../../store/store";

export const ElementalPages: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const pageManager = useSelector((state: storeState) => state.PageManager);
  return (
    <>
      <section className="flex">
        <button>First</button>
        <button>Prev</button>
        {[...Array(pageManager.length)].map((sth, i) => {
          return <button key={i}>{i}</button>;
        })}
        <button>Next</button>
        <button>Last</button>
      </section>
      <section>{children}</section>
      <section>LIST</section>
    </>
  );
};
