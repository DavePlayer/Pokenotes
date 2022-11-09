import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useSearchParams } from "react-router-dom";
import { storeState } from "./../../store/store";
import { PageManagerActions } from "./../../store/actionTypes"

export const ElementalPages: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [searchParams, setSearchParams] = useSearchParams();
  // const [pageManager, setPageManager] = useState({
  //   selectedPage: parseInt(searchParams.get("p") as string) || 0,
  //   length: 5,
  //   startPage: 0,
  //   endPage: 1,
  // });
  // move pagemanager to redux you aidiot
  const pageManager = useSelector((store: storeState) => store.pageManager)
  const dispatch = useDispatch()

  const nextPage = () => {
    dispatch({ type: PageManagerActions.NEXT })
  }
  const prevPage = () => {
    dispatch({ type: PageManagerActions.PREV })
  }
  const lastPage = () => {
    dispatch({ type: PageManagerActions.LAST })
  }
  const firstPage = () => {
    dispatch({ type: PageManagerActions.FIRST })
  }
  const pageByIndex = (index: number) => {
    dispatch({ type: PageManagerActions.INDEX, payload: index })
  }
  useEffect(() => {
    console.log("page updated");
  }, [pageManager.selectedPage]);


  return (
    <>
      <section className="flex center justify-center">
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == 0 ? true : false}
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
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == 0 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              prev.set("p", `${parseInt(prev.get("p") as string) - 1}`);
              return prev;
            })
          }
        >
          Prev
        </button>
        {[...Array(pageManager.length)].map((sth, i) => {
          return (
            <button
              className={`page-selector numeric ${parseInt(searchParams.get("p") as string) == i + 1
                ? "selected"
                : searchParams.get("p") == null && i == 0
                  ? "selected"
                  : ""
                }`}
              key={i}
              onClick={() =>
                setSearchParams((prev) => {
                  // indexed
                  setPageManager((prev) => {
                    prev.selectedPage = i;
                    return prev;
                  });
                  prev.set("p", `${i + 1}`);
                  return prev;
                })
              }
            >
              {i + 1}
            </button>
          );
        })}
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == pageManager.length - 1 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              prev.set("p", `${parseInt(prev.get("p") as string) + 1}`);
              nextPage()
              return prev;
            })
          }
        >
          Next
        </button>
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == pageManager.length - 1 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              lastPage()
              prev.set("p", `${pageManager.endPage}`);
              return prev;
            })
          }
        >
          Last
        </button>
      </section>
      <section>{children}</section>
      <section className="flex center justify-center">
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == 0 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              prev.set("p", `${1}`);
              firstPage()
              return prev;
            })
          }
        >
          First
        </button>
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == 0 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              prev.set("p", `${parseInt(prev.get("p") as string) - 1}`);
              setPageManager((prevv) => {
                prevv.selectedPage = parseInt(prev.get("p") as string) - 1;
                return prevv;
              });
              return prev;
            })
          }
        >
          Prev
        </button>
        {[...Array(pageManager.length)].map((sth, i) => {
          return (
            <button
              className={`page-selector numeric ${parseInt(searchParams.get("p") as string) == i + 1
                ? "selected"
                : searchParams.get("p") == null && i == 0
                  ? "selected"
                  : ""
                }`}
              key={i}
              onClick={() =>
                setSearchParams((prev) => {
                  setPageManager((prev) => {
                    prev.selectedPage = i;
                    return prev;
                  });
                  prev.set("p", `${i + 1}`);
                  return prev;
                })
              }
            >
              {i + 1}
            </button>
          );
        })}
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == pageManager.length - 1 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              prev.set("p", `${parseInt(prev.get("p") as string) + 1}`);
              setPageManager((prevv) => {
                prevv.selectedPage = parseInt(prev.get("p") as string) - 1;
                return prevv;
              });
              return prev;
            })
          }
        >
          Next
        </button>
        <button
          className="page-selector"
          disabled={pageManager.selectedPage == pageManager.length - 1 ? true : false}
          onClick={() =>
            setSearchParams((prev) => {
              setPageManager((prevv) => {
                prevv.selectedPage = prevv.length - 1;
                prev.set("p", `${prevv.length}`);
                return prevv;
              });
              return prev;
            })
          }
        >
          Last
        </button>
      </section>
    </>
  );
};
