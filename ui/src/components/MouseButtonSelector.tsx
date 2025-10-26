import { MouseButton } from "@/lib/tauri";

interface MouseButtonSelectorProps {
  selectedButton: MouseButton | null;
  onButtonSelect: (button: MouseButton) => void;
}

export function MouseButtonSelector({ selectedButton, onButtonSelect }: MouseButtonSelectorProps) {
  const buttons: { value: MouseButton; label: string; color: string }[] = [
    { value: "left" as MouseButton, label: "Left Click", color: "from-blue-500 to-blue-600" },
    { value: "right" as MouseButton, label: "Right Click", color: "from-purple-500 to-purple-600" },
    { value: "middle" as MouseButton, label: "Middle Click", color: "from-green-500 to-green-600" },
  ];

  return (
    <div className="relative">
      {/* Mouse Visual */}
      <div className="flex justify-center mb-6">
        <div className="relative w-48 h-64">
          {/* Mouse Body */}
          <svg
            viewBox="0 0 200 280"
            className="w-full h-full drop-shadow-2xl"
            xmlns="http://www.w3.org/2000/svg"
          >
            {/* Mouse outline */}
            <defs>
              <linearGradient id="mouseGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" stopColor="#374151" />
                <stop offset="100%" stopColor="#1f2937" />
              </linearGradient>
              <filter id="glow">
                <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
                <feMerge>
                  <feMergeNode in="coloredBlur"/>
                  <feMergeNode in="SourceGraphic"/>
                </feMerge>
              </filter>
            </defs>

            {/* Mouse body */}
            <rect
              x="20"
              y="20"
              width="160"
              height="240"
              rx="80"
              fill="url(#mouseGradient)"
              stroke="#4b5563"
              strokeWidth="2"
            />

            {/* Left button */}
            <path
              d="M 20 100 L 20 60 Q 20 20 60 20 L 95 20 L 95 100 Z"
              fill={selectedButton === "left" ? "#3b82f6" : "#4b5563"}
              stroke="#6b7280"
              strokeWidth="2"
              className="cursor-pointer transition-all duration-300 hover:opacity-80"
              onClick={() => onButtonSelect("left" as MouseButton)}
              filter={selectedButton === "left" ? "url(#glow)" : ""}
            />

            {/* Right button */}
            <path
              d="M 180 100 L 180 60 Q 180 20 140 20 L 105 20 L 105 100 Z"
              fill={selectedButton === "right" ? "#a855f7" : "#4b5563"}
              stroke="#6b7280"
              strokeWidth="2"
              className="cursor-pointer transition-all duration-300 hover:opacity-80"
              onClick={() => onButtonSelect("right" as MouseButton)}
              filter={selectedButton === "right" ? "url(#glow)" : ""}
            />

            {/* Middle button/scroll wheel */}
            <rect
              x="85"
              y="40"
              width="30"
              height="50"
              rx="15"
              fill={selectedButton === "middle" ? "#22c55e" : "#374151"}
              stroke="#6b7280"
              strokeWidth="2"
              className="cursor-pointer transition-all duration-300 hover:opacity-80"
              onClick={() => onButtonSelect("middle" as MouseButton)}
              filter={selectedButton === "middle" ? "url(#glow)" : ""}
            />

            {/* Scroll wheel lines */}
            <line x1="100" y1="50" x2="100" y2="55" stroke="#6b7280" strokeWidth="2" />
            <line x1="100" y1="60" x2="100" y2="65" stroke="#6b7280" strokeWidth="2" />
            <line x1="100" y1="70" x2="100" y2="75" stroke="#6b7280" strokeWidth="2" />
          </svg>
        </div>
      </div>

      {/* Button Selection Cards */}
      <div className="grid grid-cols-3 gap-3">
        {buttons.map((button) => {
          const isSelected = selectedButton === button.value;
          return (
            <button
              key={button.value}
              onClick={() => onButtonSelect(button.value)}
              className={`
                relative overflow-hidden rounded-lg p-4 text-center transition-all duration-300
                ${
                  isSelected
                    ? `bg-gradient-to-br ${button.color} text-white shadow-lg scale-105`
                    : "bg-muted hover:bg-muted/80 text-muted-foreground hover:scale-102"
                }
              `}
            >
              <div className="relative z-10">
                <div className="text-sm font-semibold">{button.label}</div>
                {isSelected && (
                  <div className="mt-1 text-xs opacity-90">Selected</div>
                )}
              </div>
              {isSelected && (
                <div className="absolute inset-0 bg-gradient-to-br from-white/20 to-transparent" />
              )}
            </button>
          );
        })}
      </div>

      {/* Helper Text */}
      <p className="mt-4 text-center text-xs text-muted-foreground">
        Click on the mouse or buttons below to select
      </p>
    </div>
  );
}

