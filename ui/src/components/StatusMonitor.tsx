import { useEffect, useState } from "react";
import { MacroStatus, getMacroStatus, toggleMacro } from "@/lib/tauri";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Switch } from "@/components/ui/switch";
import { Badge } from "@/components/ui/badge";
import { Activity, Pause, Play, Keyboard, Mouse } from "lucide-react";

interface StatusMonitorProps {
  isRunning: boolean;
}

export function StatusMonitor({ isRunning }: StatusMonitorProps) {
  const [statuses, setStatuses] = useState<MacroStatus[]>([]);

  useEffect(() => {
    if (!isRunning) {
      setStatuses([]);
      return;
    }

    // Initial fetch
    fetchStatus();

    // Poll every second
    const interval = setInterval(fetchStatus, 1000);
    return () => clearInterval(interval);
  }, [isRunning]);

  const fetchStatus = async () => {
    try {
      const status = await getMacroStatus();
      setStatuses(status);
    } catch (error) {
      console.error("Failed to fetch macro status:", error);
    }
  };

  const handleToggle = async (index: number) => {
    try {
      await toggleMacro(index);
      await fetchStatus();
    } catch (error) {
      console.error("Failed to toggle macro:", error);
    }
  };

  if (!isRunning) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Pause className="h-5 w-5" />
            Status Monitor
          </CardTitle>
          <CardDescription>Macro engine is not running</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            Start the macro engine to see real-time status
          </p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Activity className="h-5 w-5 animate-pulse text-green-500" />
          Status Monitor
          <Badge variant="outline" className="ml-auto">
            {statuses.filter((s) => s.enabled).length} / {statuses.length} Active
          </Badge>
        </CardTitle>
        <CardDescription>Real-time macro status</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-3">
          {statuses.map((status) => (
            <div
              key={status.index}
              className="flex items-center justify-between rounded-lg border p-3"
            >
              <div className="flex items-center gap-3">
                <div
                  className={`h-3 w-3 rounded-full ${
                    status.enabled ? "bg-green-500 animate-pulse" : "bg-gray-300"
                  }`}
                />
                {status.action_type === "keyboard" ? (
                  <Keyboard className="h-4 w-4 text-blue-500" />
                ) : (
                  <Mouse className="h-4 w-4 text-purple-500" />
                )}
                <div>
                  <p className="text-sm font-medium">
                    Macro #{status.index + 1}
                  </p>
                  {status.action_type === "keyboard" ? (
                    <p className="text-xs text-muted-foreground">
                      Key: <kbd className="rounded bg-muted px-1.5 py-0.5">{status.key.toUpperCase()}</kbd>
                      {" → "}
                      Hotkey: <kbd className="rounded bg-muted px-1.5 py-0.5">{status.toggle_hotkey}</kbd>
                    </p>
                  ) : (
                    <p className="text-xs text-muted-foreground">
                      Button: <kbd className="rounded bg-muted px-1.5 py-0.5 capitalize">{status.mouse_button}</kbd>
                      {" → "}
                      Hotkey: <kbd className="rounded bg-muted px-1.5 py-0.5">{status.toggle_hotkey}</kbd>
                    </p>
                  )}
                </div>
              </div>
              <div className="flex items-center gap-2">
                <Badge variant={status.enabled ? "default" : "secondary"}>
                  {status.enabled ? (
                    <>
                      <Play className="mr-1 h-3 w-3" />
                      Running
                    </>
                  ) : (
                    <>
                      <Pause className="mr-1 h-3 w-3" />
                      Paused
                    </>
                  )}
                </Badge>
                <Switch
                  checked={status.enabled}
                  onCheckedChange={() => handleToggle(status.index)}
                />
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

