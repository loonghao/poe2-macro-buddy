import { cn } from "@/lib/utils";

interface KeyboardMapProps {
  selectedKey?: string;
  onKeySelect: (key: string) => void;
  disabledKeys?: string[];
}

// Keyboard layout definition
const KEYBOARD_LAYOUT = [
  // Number row
  ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
  // Top row
  ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
  // Middle row
  ["a", "s", "d", "f", "g", "h", "j", "k", "l"],
  // Bottom row
  ["z", "x", "c", "v", "b", "n", "m"],
];

// Function keys
const FUNCTION_KEYS = ["F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];

export function KeyboardMap({ selectedKey, onKeySelect, disabledKeys = [] }: KeyboardMapProps) {
  const isKeyDisabled = (key: string) => disabledKeys.includes(key);
  const isKeySelected = (key: string) => selectedKey === key;

  return (
    <div className="space-y-6">
      {/* Function Keys */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-muted-foreground">Function Keys (Hotkeys)</h3>
        <div className="grid grid-cols-12 gap-1">
          {FUNCTION_KEYS.map((key) => (
            <button
              key={key}
              onClick={() => !isKeyDisabled(key) && onKeySelect(key)}
              disabled={isKeyDisabled(key)}
              className={cn(
                "h-10 rounded-md border-2 text-xs font-medium transition-all",
                "hover:scale-105 active:scale-95",
                isKeySelected(key)
                  ? "border-primary bg-primary text-primary-foreground shadow-lg"
                  : "border-border bg-background hover:border-primary/50 hover:bg-accent",
                isKeyDisabled(key) && "cursor-not-allowed opacity-40 hover:scale-100"
              )}
            >
              {key}
            </button>
          ))}
        </div>
      </div>

      {/* Main Keyboard */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-muted-foreground">Keyboard Keys (Target Keys)</h3>
        <div className="inline-block rounded-lg border-2 border-border bg-muted/30 p-4">
          <div className="space-y-2">
            {KEYBOARD_LAYOUT.map((row, rowIndex) => (
              <div
                key={rowIndex}
                className="flex justify-center gap-2"
                style={{
                  paddingLeft: rowIndex === 2 ? "1.5rem" : rowIndex === 3 ? "3rem" : "0",
                }}
              >
                {row.map((key) => (
                  <button
                    key={key}
                    onClick={() => !isKeyDisabled(key) && onKeySelect(key)}
                    disabled={isKeyDisabled(key)}
                    className={cn(
                      "h-12 w-12 rounded-md border-2 text-sm font-semibold uppercase transition-all",
                      "hover:scale-110 active:scale-95",
                      isKeySelected(key)
                        ? "border-primary bg-primary text-primary-foreground shadow-lg"
                        : "border-border bg-background hover:border-primary/50 hover:bg-accent",
                      isKeyDisabled(key) && "cursor-not-allowed opacity-40 hover:scale-100"
                    )}
                  >
                    {key}
                  </button>
                ))}
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center gap-4 text-xs text-muted-foreground">
        <div className="flex items-center gap-2">
          <div className="h-4 w-4 rounded border-2 border-primary bg-primary" />
          <span>Selected</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="h-4 w-4 rounded border-2 border-border bg-background" />
          <span>Available</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="h-4 w-4 rounded border-2 border-border bg-background opacity-40" />
          <span>In Use</span>
        </div>
      </div>
    </div>
  );
}

