import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { AuthProvider } from './Auth/AuthContext.tsx'
import { AudioPlaybackCacheProvider } from './Providers/AudioPlaybackCacheContext.tsx'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
const queryClient = new QueryClient();


createRoot(document.getElementById('root')!).render(
  <StrictMode>
     <QueryClientProvider client={queryClient}>
    <AuthProvider>
      <AudioPlaybackCacheProvider>
        <App />
      </AudioPlaybackCacheProvider>
      
    </AuthProvider>
   </QueryClientProvider>
  </StrictMode>,
)
