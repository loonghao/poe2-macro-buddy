import { useState } from "react";
import { KeyMacro } from "@/lib/tauri";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import { KeyboardMap } from "./KeyboardMap";
import { MouseButtonSelector } from "./MouseButtonSelector";
import { Trash2, Plus, Keyboard, Mouse } from "lucide-react";

interface MacroConfigProps {
  macros: KeyMacro[];
  onChange: (macros: KeyMacro[]) => void;
}

export function MacroConfig({ macros, onChange }: MacroConfigProps) {
  const [editingIndex, setEditingIndex] = useState<number | null>(null);

  const usedKeys = macros.map((m) => m.key);
  const usedHotkeys = macros.map((m) => m.toggle_hotkey);

  const addMacro = () => {
    const newMacro: KeyMacro = {
      action_type: "keyboard",
      key: "",
      interval_ms: 1000,
      random_variance_ms: 200,
      toggle_hotkey: "",
      enabled_by_default: false,
    };
    onChange([...macros, newMacro]);
    setEditingIndex(macros.length);
  };

  const removeMacro = (index: number) => {
    onChange(macros.filter((_, i) => i !== index));
    if (editingIndex === index) {
      setEditingIndex(null);
    }
  };

  const updateMacro = (index: number, updates: Partial<KeyMacro>) => {
    const updated = macros.map((macro, i) =>
      i === index ? { ...macro, ...updates } : macro
    );
    onChange(updated);
  };

  return (
    <div className="space-y-4">
      {/* Macro List */}
      <div className="space-y-2">
        {macros.map((macro, index) => (
          <Card
            key={index}
            className={editingIndex === index ? "border-primary" : ""}
          >
            <CardHeader className="pb-3">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <CardTitle className="text-base flex items-center gap-2">
                    {macro.action_type === "keyboard" ? (
                      <Keyboard className="h-4 w-4 text-blue-500" />
                    ) : (
                      <Mouse className="h-4 w-4 text-purple-500" />
                    )}
                    Macro #{index + 1}
                    {macro.action_type === "keyboard" && macro.key && macro.toggle_hotkey && (
                      <span className="ml-2 text-sm font-normal text-muted-foreground">
                        Key: <kbd className="rounded bg-muted px-2 py-1">{macro.key.toUpperCase()}</kbd>
                        {" → "}
                        Hotkey: <kbd className="rounded bg-muted px-2 py-1">{macro.toggle_hotkey}</kbd>
                      </span>
                    )}
                    {macro.action_type === "mouse" && macro.mouse_button && macro.toggle_hotkey && (
                      <span className="ml-2 text-sm font-normal text-muted-foreground">
                        Button: <kbd className="rounded bg-muted px-2 py-1 capitalize">{macro.mouse_button}</kbd>
                        {" → "}
                        Hotkey: <kbd className="rounded bg-muted px-2 py-1">{macro.toggle_hotkey}</kbd>
                      </span>
                    )}
                  </CardTitle>
                  <CardDescription>
                    {macro.interval_ms}ms ± {macro.random_variance_ms}ms
                    {" "}({macro.interval_ms - macro.random_variance_ms}-
                    {macro.interval_ms + macro.random_variance_ms}ms)
                  </CardDescription>
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    variant={editingIndex === index ? "default" : "outline"}
                    size="sm"
                    onClick={() => setEditingIndex(editingIndex === index ? null : index)}
                  >
                    {editingIndex === index ? "Done" : "Edit"}
                  </Button>
                  <Button
                    variant="destructive"
                    size="sm"
                    onClick={() => removeMacro(index)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            </CardHeader>

            {editingIndex === index && (
              <CardContent className="space-y-6">
                {/* Action Type Selection */}
                <div className="space-y-2">
                  <Label>Action Type</Label>
                  <div className="grid grid-cols-2 gap-2">
                    <button
                      onClick={() => updateMacro(index, { action_type: "keyboard", mouse_button: undefined })}
                      className={`
                        flex items-center justify-center gap-2 rounded-lg p-4 transition-all
                        ${
                          macro.action_type === "keyboard"
                            ? "bg-blue-500 text-white shadow-lg"
                            : "bg-muted hover:bg-muted/80"
                        }
                      `}
                    >
                      <Keyboard className="h-5 w-5" />
                      <span className="font-medium">Keyboard</span>
                    </button>
                    <button
                      onClick={() => updateMacro(index, { action_type: "mouse", key: "" })}
                      className={`
                        flex items-center justify-center gap-2 rounded-lg p-4 transition-all
                        ${
                          macro.action_type === "mouse"
                            ? "bg-purple-500 text-white shadow-lg"
                            : "bg-muted hover:bg-muted/80"
                        }
                      `}
                    >
                      <Mouse className="h-5 w-5" />
                      <span className="font-medium">Mouse</span>
                    </button>
                  </div>
                </div>

                {/* Key Selection (Keyboard) */}
                {macro.action_type === "keyboard" && (
                  <div className="space-y-2">
                    <Label>Target Key (Key to Press)</Label>
                    <KeyboardMap
                      selectedKey={macro.key}
                      onKeySelect={(key) => updateMacro(index, { key })}
                      disabledKeys={usedKeys.filter((k) => k !== macro.key)}
                    />
                  </div>
                )}

                {/* Mouse Button Selection (Mouse) */}
                {macro.action_type === "mouse" && (
                  <div className="space-y-2">
                    <Label>Mouse Button (Button to Click)</Label>
                    <MouseButtonSelector
                      selectedButton={macro.mouse_button || null}
                      onButtonSelect={(button) => updateMacro(index, { mouse_button: button })}
                    />
                  </div>
                )}

                {/* Hotkey Selection */}
                <div className="space-y-2">
                  <Label>Toggle Hotkey (Press to Enable/Disable)</Label>
                  <div className="grid grid-cols-12 gap-1">
                    {["F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"].map((key) => {
                      const isUsed = usedHotkeys.includes(key) && macro.toggle_hotkey !== key;
                      const isSelected = macro.toggle_hotkey === key;
                      return (
                        <button
                          key={key}
                          onClick={() => !isUsed && updateMacro(index, { toggle_hotkey: key })}
                          disabled={isUsed}
                          className={`h-10 rounded-md border-2 text-xs font-medium transition-all ${
                            isSelected
                              ? "border-primary bg-primary text-primary-foreground"
                              : isUsed
                              ? "cursor-not-allowed opacity-40"
                              : "border-border bg-background hover:border-primary/50"
                          }`}
                        >
                          {key}
                        </button>
                      );
                    })}
                  </div>
                </div>

                {/* Interval Settings */}
                <div className="space-y-4">
                  <div className="space-y-2">
                    <Label>Base Interval: {macro.interval_ms}ms</Label>
                    <Slider
                      value={[macro.interval_ms]}
                      onValueChange={([value]) => updateMacro(index, { interval_ms: value })}
                      min={100}
                      max={5000}
                      step={100}
                    />
                    <p className="text-xs text-muted-foreground">
                      Time between {macro.action_type === "keyboard" ? "key presses" : "mouse clicks"} (100ms - 5000ms)
                    </p>
                  </div>

                  <div className="space-y-2">
                    <Label>Random Variance: ±{macro.random_variance_ms}ms</Label>
                    <Slider
                      value={[macro.random_variance_ms]}
                      onValueChange={([value]) => updateMacro(index, { random_variance_ms: value })}
                      min={0}
                      max={1000}
                      step={50}
                    />
                    <p className="text-xs text-muted-foreground">
                      Randomness to make it more human-like (0ms - 1000ms)
                    </p>
                  </div>
                </div>

                {/* Enabled by Default */}
                <div className="flex items-center justify-between rounded-lg border p-4">
                  <div className="space-y-0.5">
                    <Label>Enabled by Default</Label>
                    <p className="text-sm text-muted-foreground">
                      Start this macro automatically when the app launches
                    </p>
                  </div>
                  <Switch
                    checked={macro.enabled_by_default}
                    onCheckedChange={(checked) =>
                      updateMacro(index, { enabled_by_default: checked })
                    }
                  />
                </div>
              </CardContent>
            )}
          </Card>
        ))}
      </div>

      {/* Add Macro Button */}
      <Button onClick={addMacro} variant="outline" className="w-full">
        <Plus className="mr-2 h-4 w-4" />
        Add New Macro
      </Button>
    </div>
  );
}

