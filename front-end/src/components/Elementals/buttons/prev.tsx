
import { useSelector } from "react-redux"
import { useSearchParams } from "react-router-dom";
import { storeState } from "../../../store/store"

export const PrevButton: React.FC = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const pageManager = useSelector((store: storeState) => store.pageManager)
    return (
        <button
            className="page-selector"
            disabled={pageManager.selectedPage <= 1 ? true : false}
            onClick={() =>
                setSearchParams((prev) => {
                    prev.set("p", `${parseInt(prev.get("p") as string) - 1}`);
                    return prev;
                })
            }
        >
            Prev
        </button>
    )
}