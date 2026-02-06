export function TransformStorePanel() {
  return (
    <div className="h-full w-full p-4 overflow-auto text-sm text-gray-700">
      <div className="font-semibold mb-3">Transform Store</div>

      <div className="space-y-2">
        <div className="p-2 bg-white border rounded cursor-pointer hover:bg-gray-100">
          Gain
        </div>
        <div className="p-2 bg-white border rounded cursor-pointer hover:bg-gray-100">
          Filter
        </div>
        <div className="p-2 bg-white border rounded cursor-pointer hover:bg-gray-100">
          Mixer
        </div>
      </div>
    </div>
  );
}