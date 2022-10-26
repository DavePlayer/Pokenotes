import { useEffect, useState } from "react";

interface IListElement {
    name: String;
    image?: String
}



export const ElementalList = () => {
    const [list, setList] = useState<Array<IListElement>>([])
    useEffect(() => {
        for (let i = 0; i < 50; i++) {
            setList(prev => prev = [...prev, { name: "dummy data" }])
        }
    }, [])

    return (
        <>
            <section className="elemental-list">
                {
                    list.map((element: IListElement) => (
                        <article className="elemental-list-element">{element.name}</article>
                    ))
                }
            </section>
        </>
    )
}