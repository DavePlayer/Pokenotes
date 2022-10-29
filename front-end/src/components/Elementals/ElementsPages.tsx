import React from "react";
import { ElementalList } from "./ElementalList";

export const ElementalPages: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <>
      <section>LIST</section>
      <section>{children}</section>
      <section>LIST</section>
    </>
  );
};
