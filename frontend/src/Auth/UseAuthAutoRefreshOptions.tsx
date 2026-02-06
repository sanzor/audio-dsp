import { useEffect, useRef } from "react";
import { useAuth } from "./UseAuth";

interface UseTokenAutoRefreshOptions {
  /**
   * Whether auto-refreshing is active
   */
  enabled?: boolean;

  /**
   * Interval in milliseconds to refresh token
   * Default: 13 minutes (before token expires)
   */
  interval?: number;
}
export function useTokenAutoRefresh({
  enabled = true,
  interval = 13 * 60 * 1000, // 13 minutes
}: UseTokenAutoRefreshOptions = {}) {
  const { refreshToken,user } = useAuth();
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const isRefreshingRef = useRef(false);
  // console.log('🔄 useTokenAutoRefresh called - enabled:', enabled, 'user:', user?.name || 'null');
  useEffect(() => {
    // console.log(user);
     if (!enabled || !user) {
      console.log('❌ Auto-refresh disabled or no user, clearing interval');
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      return;
    }
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

       // For development: test refresh after 5 seconds
    if (process.env.NODE_ENV === 'development') {
      // console.log('🧪 Development mode: will test refresh in 5 seconds');
      setTimeout(async () => {
        if (!isRefreshingRef.current && user) {
          // console.log('🧪 Testing token refresh...');
          isRefreshingRef.current = true;
          try {
            await refreshToken();
            // console.log('✅ Test refresh successful');
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          } catch (error) {
            // console.error('❌ Test refresh failed:', error);
          } finally {
            isRefreshingRef.current = false;
          }
        }
      }, 5000);
    }
    intervalRef.current = setInterval(async () => {
      if (isRefreshingRef.current) {
        // console.log('⏭️ Refresh already in progress, skipping');
        return;
      }

      // console.log('🔄 Auto-refresh triggered');
      isRefreshingRef.current = true;

      try {
        await refreshToken();
        // console.log('✅ Auto-refresh successful');
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      } catch (error) {
        // console.error('❌ Auto-refresh failed:', error);
        // You might want to handle this by redirecting to login
        // or showing a notification to the user
      } finally {
        isRefreshingRef.current = false;
      }
    }, interval);

   return () => {
      // console.log('🧹 Cleaning up auto-refresh interval');
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      isRefreshingRef.current = false;
    };
  }, [enabled, interval, refreshToken, user]);
   useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, []);
}
