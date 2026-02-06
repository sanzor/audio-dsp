import './App.css'
import { AuthCallback } from './Auth/AuthCallback';
import { useTokenAutoRefresh } from './Auth/UseAuthAutoRefreshOptions'
import { Dashboard } from './components/dashboard/dashboard';
import { LoginForm } from './components/login-form'
import { useAuth } from './Auth/UseAuth';
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate
} from "react-router-dom";
import { AuthListener } from './Auth/AuthListener';

console.log('📁 App.js file loaded');

function AppContent() {
  // console.log('🏠 AppContent component rendering...');
  
  const authResult = useAuth();
  // console.log('🔐 useAuth result:', authResult);
  
  const { user, loading } = authResult;
  
  // console.log('👤 User state:', user);
  // console.log('⏳ Loading state:', loading);

  // Call the auto-refresh hook
  // console.log('🔄 About to call useTokenAutoRefresh...');
  useTokenAutoRefresh({
    enabled: true,
    interval: 2 * 60 * 1000, // 2 minutes for testing
  });
  // console.log('✅ useTokenAutoRefresh called');

  // console.log('🏠 App render - user:', user?.name || 'null', 'loading:', loading);

  if (loading) {
    // console.log('⏳ Still loading, showing loading screen');
    return <div>Loading...</div>;
  }

  // console.log('✅ Not loading, rendering routes');

  return (
    <>
    <AuthListener/>
   
    <Routes>
      <Route path="/auth/callback" element={<AuthCallback />} />
      <Route 
        path="/login" 
        element={user ? <Navigate to="/dashboard" replace /> : <LoginForm />} 
      />
      <Route 
        path="/dashboard" 
        element={user ? <Dashboard /> : <Navigate to="/login" replace />} 
      />
      <Route 
        path="/" 
        element={user ? <Navigate to="/dashboard" replace /> : <Navigate to="/login" replace />} 
      />
    </Routes>
    </>
   
  );
}

function App() {
  console.log('🚀 App component rendering...');
  
  return (
    <Router>
      <AppContent />
    </Router>
  );
}

console.log('📁 App.js export ready');

export default App