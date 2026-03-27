import { useTransformStore } from "@/Stores/TransformStore";
import { useListTransforms } from "@/hooks/transforms/queries";
import { TransformItem } from "./TransformItem";

export function TransformStorePanel() {
  const { fetchNextPage, hasNextPage, isFetchingNextPage } = useListTransforms();
  const transforms = Array.from(useTransformStore((s) => s.transforms).values());

  return (
    <>
      <div className="panel-header text-center text-sm">Store</div>
      <div className="panel-content px-1">
        {transforms.map((t) => (
          <TransformItem key={t.transform_id} transform={t} />
        ))}

        {hasNextPage && (
          <button
            className="store-btn text-xs"
            onClick={() => fetchNextPage()}
            disabled={isFetchingNextPage}
          >
            {isFetchingNextPage ? "..." : "More"}
          </button>
        )}
      </div>
    </>
  );
}
