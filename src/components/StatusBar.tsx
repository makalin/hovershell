import React, { useState, useEffect } from 'react';
import { 
  Wifi, 
  WifiOff, 
  Battery, 
  BatteryCharging, 
  Clock, 
  GitBranch, 
  Terminal,
  Zap,
  Settings
} from 'lucide-react';
import { useAppStore } from '../stores/appStore';
import { useTerminalStore } from '../stores/terminalStore';

export const StatusBar: React.FC = () => {
  const [currentTime, setCurrentTime] = useState(new Date());
  const [isOnline, setIsOnline] = useState(navigator.onLine);
  const [batteryLevel, setBatteryLevel] = useState<number | null>(null);
  const [isCharging, setIsCharging] = useState(false);
  
  const { systemInfo, providers, defaultProvider } = useAppStore();
  const { terminals, activeTerminalId } = useTerminalStore();

  // Update time every second
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  // Monitor online status
  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  // Monitor battery status
  useEffect(() => {
    if ('getBattery' in navigator) {
      (navigator as any).getBattery().then((battery: any) => {
        setBatteryLevel(Math.round(battery.level * 100));
        setIsCharging(battery.charging);

        const handleBatteryChange = () => {
          setBatteryLevel(Math.round(battery.level * 100));
          setIsCharging(battery.charging);
        };

        battery.addEventListener('levelchange', handleBatteryChange);
        battery.addEventListener('chargingchange', handleBatteryChange);

        return () => {
          battery.removeEventListener('levelchange', handleBatteryChange);
          battery.removeEventListener('chargingchange', handleBatteryChange);
        };
      });
    }
  }, []);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { 
      hour: '2-digit', 
      minute: '2-digit',
      hour12: false 
    });
  };

  const formatDate = (date: Date) => {
    return date.toLocaleDateString([], { 
      month: 'short', 
      day: 'numeric' 
    });
  };

  const getBatteryIcon = () => {
    if (batteryLevel === null) return null;
    
    if (isCharging) {
      return <BatteryCharging size={16} className="text-green-500" />;
    }
    
    if (batteryLevel > 75) {
      return <Battery size={16} className="text-green-500" />;
    } else if (batteryLevel > 25) {
      return <Battery size={16} className="text-yellow-500" />;
    } else {
      return <Battery size={16} className="text-red-500" />;
    }
  };

  const getNetworkIcon = () => {
    return isOnline ? (
      <Wifi size={16} className="text-green-500" />
    ) : (
      <WifiOff size={16} className="text-red-500" />
    );
  };

  const activeTerminal = terminals.find(t => t.id === activeTerminalId);
  const defaultProviderInfo = providers.find(p => p.id === defaultProvider);

  return (
    <div className="status-bar h-8 bg-gray-900 border-t border-gray-700 flex items-center justify-between px-4 text-sm">
      {/* Left side */}
      <div className="flex items-center space-x-4">
        {/* Terminal info */}
        <div className="flex items-center space-x-2">
          <Terminal size={14} className="text-gray-400" />
          <span className="text-gray-300">
            {terminals.length} terminal{terminals.length !== 1 ? 's' : ''}
          </span>
          {activeTerminal && (
            <span className="text-gray-500">
              • {activeTerminal.title}
            </span>
          )}
        </div>

        {/* AI Provider */}
        {defaultProviderInfo && (
          <div className="flex items-center space-x-2">
            <Zap size={14} className="text-yellow-500" />
            <span className="text-gray-300">{defaultProviderInfo.name}</span>
          </div>
        )}

        {/* Git branch */}
        <div className="flex items-center space-x-2">
          <GitBranch size={14} className="text-gray-400" />
          <span className="text-gray-500">main</span>
        </div>
      </div>

      {/* Right side */}
      <div className="flex items-center space-x-4">
        {/* System info */}
        {systemInfo && (
          <div className="flex items-center space-x-2 text-gray-500">
            <span>{systemInfo.os}</span>
            <span>•</span>
            <span>{systemInfo.arch}</span>
          </div>
        )}

        {/* Network status */}
        <div className="flex items-center space-x-2">
          {getNetworkIcon()}
          <span className="text-gray-300">
            {isOnline ? 'Online' : 'Offline'}
          </span>
        </div>

        {/* Battery */}
        {batteryLevel !== null && (
          <div className="flex items-center space-x-2">
            {getBatteryIcon()}
            <span className="text-gray-300">{batteryLevel}%</span>
          </div>
        )}

        {/* Time */}
        <div className="flex items-center space-x-2">
          <Clock size={14} className="text-gray-400" />
          <span className="text-gray-300">{formatTime(currentTime)}</span>
          <span className="text-gray-500">{formatDate(currentTime)}</span>
        </div>

        {/* Settings */}
        <button className="p-1 hover:bg-gray-700 rounded">
          <Settings size={14} className="text-gray-400" />
        </button>
      </div>

      <style>{`
        .status-bar {
          background-color: var(--bg-color);
          border-color: var(--border-color);
          color: var(--fg-color);
        }
        
        .status-bar button:hover {
          background-color: rgba(255, 255, 255, 0.1);
        }
      `}</style>
    </div>
  );
};