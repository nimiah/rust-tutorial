"use client";
import { useCounterStore } from "@/store/counter";
import { Button } from "./ui/button";
import { Card, CardContent, CardFooter } from "./ui/card";
import { useCounter } from "./CountingProvider";

export function CountingControlsFromZustand() {
  const {
    counter,
    actions: { decrease, increase, reset },
  } = useCounterStore();

  return (
    <Card size="sm" className="mx-auto w-full max-w-sm">
      <CardContent className="flex flex-row justify-between">
        <div className="flex flex-col">
          <span>{`Counter from zustand: ${counter}`}</span>
        </div>
        {/* <Button variant="outline" size="sm" onClick={props.stopCounting}>
          Stop
        </Button> */}
      </CardContent>
      <CardFooter className="flex flex-row gap-5 items-center justify-center">
        <Button variant="outline" size="sm" onClick={decrease}>
          -1
        </Button>
        <Button variant="outline" size="sm" onClick={reset}>
          Reset
        </Button>
        <Button variant="outline" size="sm" onClick={increase}>
          +1
        </Button>
      </CardFooter>
    </Card>
  );
}

export function CountingControlsFromProvider() {
  const { counter, decrease, increase, reset } = useCounter();

  return (
    <Card size="sm" className="mx-auto w-full max-w-sm">
      <CardContent className="flex flex-row justify-between">
        <div className="flex flex-col">
          <span>{`Counter from provider: ${counter}`}</span>
        </div>
        {/* <Button variant="outline" size="sm" onClick={props.stopCounting}>
          Stop
        </Button> */}
      </CardContent>
      <CardFooter className="flex flex-row gap-5 items-center justify-center">
        <Button variant="outline" size="sm" onClick={decrease}>
          -1
        </Button>
        <Button variant="outline" size="sm" onClick={reset}>
          Reset
        </Button>
        <Button variant="outline" size="sm" onClick={increase}>
          +1
        </Button>
      </CardFooter>
    </Card>
  );
}
