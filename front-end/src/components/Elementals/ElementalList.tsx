import { useEffect, useState } from "react";
import { TagIcon } from "@heroicons/react/outline";

interface IListElement {
  name: String;
  image?: String;
}

export const ElementalList: React.FC<{ tableName: String }> = ({ tableName }) => {
  const [list, setList] = useState<Array<IListElement>>([]);
  useEffect(() => {
    for (let i = 0; i < 50; i++) {
      setList((prev) => (prev = [...prev, { name: "dummy data" }]));
    }
  }, []);

  return (
    <>
      <section className="elemental-list">
        {list.map((element: IListElement, i: number) => (
          <article key={i} className="elemental-list-element">
            {element.image != undefined ? <img src={element.image as string} alt="img" /> : <TagIcon />}
            <section className="p-3 bg-default-darkest text-white">{element.name}</section>
          </article>
        ))}
      </section>
    </>
  );
};
