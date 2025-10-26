import { useEffect, useState } from "react";
import { Config, loadConfig, saveConfig, startMacroEngine, stopMacroEngine, validateConfig } from "@/lib/tauri";
import { MacroConfig } from "@/components/MacroConfig";
import { StatusMonitor } from "@/components/StatusMonitor";
import { LoadingScreen } from "@/components/LoadingScreen";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useToast } from "@/hooks/use-toast";
import { Toaster } from "@/components/ui/toaster";
import { Play, Pause, Save, FolderOpen, Settings, Activity, Loader2 } from "lucide-react";

function App() {
  const [config, setConfig] = useState<Config>({ macros: [] });
  const [isRunning, setIsRunning] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [isInitializing, setIsInitializing] = useState(true);
  const { toast } = useToast();

  useEffect(() => {
    loadConfiguration(true);
  }, []);

  const loadConfiguration = async (isInitial = false) => {
    try {
      setIsLoading(true);
      const loadedConfig = await loadConfig();
      setConfig(loadedConfig);

      // Only show toast for manual reloads, not initial load
      if (!isInitial) {
        toast({
          title: "Configuration Loaded",
          description: `Loaded ${loadedConfig.macros.length} macro(s)`,
        });
      }
    } catch (error) {
      toast({
        title: "Error",
        description: `Failed to load configuration: ${error}`,
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
      if (isInitial) {
        setIsInitializing(false);
      }
    }
  };

  const saveConfiguration = async () => {
    try {
      // Validate first
      const validationError = await validateConfig(config);
      if (validationError) {
        toast({
          title: "Validation Error",
          description: validationError,
          variant: "destructive",
        });
        return;
      }

      setIsLoading(true);
      await saveConfig(config);
      toast({
        title: "Configuration Saved",
        description: `Saved ${config.macros.length} macro(s) successfully`,
      });
    } catch (error) {
      toast({
        title: "Error",
        description: `Failed to save configuration: ${error}`,
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleStart = async () => {
    try {
      // Validate before starting
      const validationError = await validateConfig(config);
      if (validationError) {
        toast({
          title: "Validation Error",
          description: validationError,
          variant: "destructive",
        });
        return;
      }

      setIsLoading(true);
      await startMacroEngine();
      setIsRunning(true);
      toast({
        title: "Macro Engine Started",
        description: "All macros are now running",
      });
    } catch (error) {
      toast({
        title: "Error",
        description: `Failed to start macro engine: ${error}`,
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleStop = async () => {
    try {
      setIsLoading(true);
      await stopMacroEngine();
      setIsRunning(false);
      toast({
        title: "Macro Engine Stopped",
        description: "All macros have been stopped",
      });
    } catch (error) {
      toast({
        title: "Error",
        description: `Failed to stop macro engine: ${error}`,
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  // Show loading screen during initialization
  if (isInitializing) {
    return <LoadingScreen message="Initializing Mouse Macro..." />;
  }

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto p-6 max-w-7xl">
        {/* Header */}
        <div className="mb-6">
          <h1 className="text-3xl font-bold tracking-tight bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 bg-clip-text text-transparent">
            Mouse & Keyboard Macro Configuration
          </h1>
          <p className="text-muted-foreground">
            Configure and manage your keyboard and mouse macros for POE2
          </p>
        </div>

        {/* Control Panel */}
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>Control Panel</CardTitle>
            <CardDescription>Manage macro engine and configuration</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex flex-wrap gap-2">
              <Button
                onClick={isRunning ? handleStop : handleStart}
                disabled={isLoading || config.macros.length === 0}
                variant={isRunning ? "destructive" : "default"}
                size="lg"
              >
                {isLoading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    {isRunning ? "Stopping..." : "Starting..."}
                  </>
                ) : isRunning ? (
                  <>
                    <Pause className="mr-2 h-4 w-4" />
                    Stop Engine
                  </>
                ) : (
                  <>
                    <Play className="mr-2 h-4 w-4" />
                    Start Engine
                  </>
                )}
              </Button>
              <Button
                onClick={saveConfiguration}
                disabled={isLoading || isRunning}
                variant="outline"
                size="lg"
              >
                {isLoading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Saving...
                  </>
                ) : (
                  <>
                    <Save className="mr-2 h-4 w-4" />
                    Save Configuration
                  </>
                )}
              </Button>
              <Button
                onClick={() => loadConfiguration(false)}
                disabled={isLoading || isRunning}
                variant="outline"
                size="lg"
              >
                {isLoading ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Loading...
                  </>
                ) : (
                  <>
                    <FolderOpen className="mr-2 h-4 w-4" />
                    Reload Configuration
                  </>
                )}
              </Button>
            </div>
            {config.macros.length === 0 && (
              <p className="mt-4 text-sm text-muted-foreground">
                Add at least one macro to start the engine
              </p>
            )}
          </CardContent>
        </Card>

        {/* Main Content */}
        <Tabs defaultValue="config" className="space-y-4">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="config">
              <Settings className="mr-2 h-4 w-4" />
              Configuration
            </TabsTrigger>
            <TabsTrigger value="status">
              <Activity className="mr-2 h-4 w-4" />
              Status Monitor
            </TabsTrigger>
          </TabsList>

          <TabsContent value="config" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Macro Configuration</CardTitle>
                <CardDescription>
                  Configure your keyboard and mouse macros with visual selection
                </CardDescription>
              </CardHeader>
              <CardContent>
                <MacroConfig macros={config.macros} onChange={(macros) => setConfig({ macros })} />
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="status">
            <StatusMonitor isRunning={isRunning} />
          </TabsContent>
        </Tabs>

        {/* Footer */}
        <div className="mt-6 text-center text-sm text-muted-foreground">
          <p>Mouse Macro v0.1.0 - POE2 Keyboard & Mouse Automation Tool</p>
          <p className="mt-1">
            ⚠️ Use responsibly and in accordance with game rules
          </p>
        </div>
      </div>
      <Toaster />
    </div>
  );
}

export default App;

