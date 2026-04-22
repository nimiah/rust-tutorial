import {
  CountingControlsFromZustand,
  CountingControlsFromProvider,
} from "./CountingControls";

export default function Counting() {
  return (
    <div className="flex flex-col gap-2">
      <CountingControlsFromProvider />
      <CountingControlsFromZustand />
    </div>
  );
}
