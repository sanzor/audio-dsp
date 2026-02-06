/* eslint-disable react-refresh/only-export-components */
import {
  getSession,
  refreshToken,
  logout,
  getGoogleOAuthUrl,
} from '../Services/AuthService';
import type { SessionResponse, RefreshResponse } from './AuthTypes';
import { createContext, useEffect, useRef, useState, type ReactNode } from 'react';
import type { User } from '@/Domain/User';

// Define context interface
interface AuthServiceContext {
  user: User | null;
  loading: boolean;
  getSession: () => Promise<SessionResponse>;
  refreshToken: () => Promise<RefreshResponse>;
  logout: () => Promise<void>;
  getGoogleOAuthUrl: () => string;
}

// Create context
export const AuthContext = createContext<AuthServiceContext | null>(null);

// Provider
export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const initRef = useRef(false);
  // console.log('🏗️ AuthProvider render - user:', user?.name || 'null', 'loading:', loading);

  const checkSession = async (): Promise<SessionResponse> => {
    // console.log('🔍 Checking session...');
    const res = await getSession();
    if (res.user) {
      // console.log('✅ Session found for user:', res.user.name);
      setUser({
        id: Number.parseInt(res.user.user_id),
        name: res.user.name,
        email: res.user.email,
        photo: res.user.photo,
      });
    } else {
      // console.log('❌ No session found');
      setUser(null);
    }
    return res;
  };

   const handleRefreshToken = async (): Promise<RefreshResponse> => {
    // console.log('🔄 RefreshToken called from context');
    try {
      const result = await refreshToken();
      // console.log('✅ Token refreshed, checking session...');
      
      // After successful refresh, update the user session
      await checkSession();
      
      return result;
    } catch (error) {
      // console.error('❌ Refresh failed, clearing user:', error);
      setUser(null);
      throw error;
    }
  };

   const handleLogout = async (): Promise<void> => {
    console.log('👋 Logout called');
    try {
      await logout();
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      setUser(null);
    }
  };
  useEffect(() => {
    if (initRef.current) return;
    initRef.current = true;
    
    console.log('🚀 AuthProvider initializing...');
    
    checkSession()
      .catch((error) => {
        console.error('❌ Initial session check failed:', error);
        setUser(null);
      })
      .finally(() => {
        console.log('🏁 AuthProvider initialization complete');
        setLoading(false);
      });
  }, []);


 const value: AuthServiceContext = {
    user,
    loading,
    getSession: checkSession,
    refreshToken: handleRefreshToken, // Use wrapped version
    logout: handleLogout,
    getGoogleOAuthUrl,
  };

 return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
