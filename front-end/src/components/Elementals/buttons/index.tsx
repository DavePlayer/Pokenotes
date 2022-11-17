
import { useDispatch, useSelector } from "react-redux"
import { useSearchParams } from "react-router-dom";
import { PageManagerActions } from "../../../store/actionTypes"
import { storeState } from "../../../store/store"

export const IndexedButton: React.FC = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const pageManager = useSelector((store: storeState) => store.pageManager)
    const dispatch = useDispatch()
    return (
        <>
            {
                [...Array(pageManager.endPage - pageManager.startPage + 1).keys()].map(x => x + pageManager.startPage).map((sth, i) => {
                    sth -= 1
                    return (
                        <button
                            className={`page-selector numeric ${parseInt(searchParams.get("p") as string) == sth + 1
                                ? "selected"
                                : searchParams.get("p") == null && sth == 0
                                    ? "selected"
                                    : ""
                                }`}
                            key={i}
                            onClick={() =>
                                setSearchParams((prev) => {
                                    // indexed
                                    prev.set("p", `${sth + 1}`);
                                    return prev;
                                })
                            }
                        >
                            {sth + 1}
                        </button>
                    );
                })
            }
        </>
    )
}