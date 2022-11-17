import { useSelector } from "react-redux"
import { useSearchParams } from "react-router-dom";
import { storeState } from "../../../store/store"

export const LastButton: React.FC = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const pageManager = useSelector((store: storeState) => store.pageManager)
    return (
        <button
            className="page-selector"
            disabled={pageManager.selectedPage >= pageManager.length ? true : false}
            onClick={() =>
                setSearchParams((prev) => {
                    prev.set("p", `${pageManager.length}`);
                    return prev;
                })
            }
        >
            Last
        </button>
    )
}