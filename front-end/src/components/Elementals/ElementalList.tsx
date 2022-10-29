import { useEffect, useState } from "react";
import { TagIcon } from "@heroicons/react/outline";
import { useSelector } from "react-redux";
import { storeState } from "../../store/store";
import { IList } from "../../store/reducers/elementalElements";

export const ElementalList: React.FC = () => {
  const list = useSelector((store: storeState) => store.elementalList);
  return (
    <>
      <section className="elemental-list">
        {(list as unknown as Array<IList>).map((element: IList, i: number) => (
          <article key={i} className="elemental-list-element">
            {element.imgUrl != undefined ? <img src={element.imgUrl as string} alt="img" /> : <TagIcon />}
            <section className="p-3 bg-default-darkest text-white">{element.name}</section>
          </article>
        ))}
      </section>
    </>
  );
};
