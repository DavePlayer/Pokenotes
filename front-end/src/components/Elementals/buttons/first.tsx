import { useDispatch, useSelector } from "react-redux"
import { useSearchParams } from "react-router-dom";
import { PageManagerActions } from "../../../store/actionTypes"
import { storeState } from "../../../store/store"

export const FirstButton: React.FC = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const pageManager = useSelector((store: storeState) => store.pageManager)
    const dispatch = useDispatch()
    const firstPage = () => {
        dispatch({ type: PageManagerActions.FIRST })
    }
    return (
        <button
            className="page-selector"
            disabled={pageManager.selectedPage <= 1 ? true : false}
            onClick={() =>
                setSearchParams((prev) => {
                    prev.set("p", `${1}`);
                    firstPage();
                    return prev;
                })
            }
        >
            First
        </button>
    )
}