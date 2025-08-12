import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface PlatformAuthStatus {
  isAuthenticated: boolean;
  userId?: string;
  email?: string;
  lastAuthenticated?: string;
  expires?: string;
  scopes?: string[];
}

interface AuthContextType {
  // Platform authentication status
  google: PlatformAuthStatus;
  slack: PlatformAuthStatus;
  github: PlatformAuthStatus;
  confluence: PlatformAuthStatus;
  
  // Authentication actions
  initiateAuth: (platform: string) => Promise<void>;
  refreshAuth: (platform: string) => Promise<void>;
  logout: (platform: string) => Promise<void>;
  logoutAll: () => Promise<void>;
  
  // Authentication status checks
  isAnyAuthenticated: () => boolean;
  getAuthenticatedPlatforms: () => string[];
  
  // Loading states
  isLoading: boolean;
  error: string | null;
}

const defaultAuthStatus: PlatformAuthStatus = {
  isAuthenticated: false,
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [google, setGoogle] = useState<PlatformAuthStatus>(defaultAuthStatus);
  const [slack, setSlack] = useState<PlatformAuthStatus>(defaultAuthStatus);
  const [github, setGithub] = useState<PlatformAuthStatus>(defaultAuthStatus);
  const [confluence, setConfluence] = useState<PlatformAuthStatus>(defaultAuthStatus);
  
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load authentication status from localStorage on component mount
  useEffect(() => {
    const loadAuthStatus = () => {
      try {
        const savedAuth = localStorage.getItem('platform_auth_status');
        if (savedAuth) {
          const authData = JSON.parse(savedAuth);
          
          setGoogle(authData.google || defaultAuthStatus);
          setSlack(authData.slack || defaultAuthStatus);
          setGithub(authData.github || defaultAuthStatus);
          setConfluence(authData.confluence || defaultAuthStatus);
          
          // Check if any tokens need refresh
          checkTokenExpiration(authData);
        }
      } catch (err) {
        console.error('Failed to load auth status from localStorage:', err);
        setError('Failed to restore authentication status');
      }
    };

    loadAuthStatus();
  }, []);

  // Save authentication status to localStorage whenever it changes
  useEffect(() => {
    const authStatus = { google, slack, github, confluence };
    localStorage.setItem('platform_auth_status', JSON.stringify(authStatus));
  }, [google, slack, github, confluence]);

  const checkTokenExpiration = (authData: any) => {
    const now = new Date();
    
    Object.keys(authData).forEach(platform => {
      const platformAuth = authData[platform];
      if (platformAuth?.expires) {
        const expiryDate = new Date(platformAuth.expires);
        if (now >= expiryDate) {
          console.warn(`${platform} token has expired`);
          // Automatically refresh if possible
          refreshAuth(platform).catch(err => 
            console.error(`Failed to refresh ${platform} token:`, err)
          );
        }
      }
    });
  };

  const initiateAuth = async (platform: string): Promise<void> => {
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`http://127.0.0.1:3001/api/auth/${platform}?user_id=current_user`);
      
      if (!response.ok) {
        throw new Error(`Failed to initiate ${platform} authentication`);
      }
      
      const data = await response.json();
      
      if (data.auth_url) {
        // Open OAuth popup window
        const popup = window.open(
          data.auth_url, 
          `${platform}_auth`, 
          'width=600,height=700,scrollbars=yes,resizable=yes'
        );
        
        if (popup) {
          // Monitor popup for completion
          const checkClosed = setInterval(() => {
            if (popup.closed) {
              clearInterval(checkClosed);
              // Check if authentication was successful
              setTimeout(() => {
                checkAuthStatus(platform);
              }, 1000);
            }
          }, 1000);
        } else {
          throw new Error('Failed to open authentication popup. Please check popup blockers.');
        }
      } else {
        throw new Error('No authentication URL received from server');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to authenticate with ${platform}`;
      setError(errorMessage);
      console.error('Authentication error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const checkAuthStatus = async (platform: string): Promise<void> => {
    try {
      const response = await fetch(`http://127.0.0.1:3001/api/auth/${platform}/status?user_id=current_user`);
      
      if (response.ok) {
        const statusData = await response.json();
        
        if (statusData.authenticated) {
          const authStatus: PlatformAuthStatus = {
            isAuthenticated: true,
            userId: statusData.user_id,
            email: statusData.email,
            lastAuthenticated: new Date().toISOString(),
            expires: statusData.expires,
            scopes: statusData.scopes,
          };
          
          switch (platform) {
            case 'google':
              setGoogle(authStatus);
              break;
            case 'slack':
              setSlack(authStatus);
              break;
            case 'github':
              setGithub(authStatus);
              break;
            case 'confluence':
              setConfluence(authStatus);
              break;
          }
          
          setError(null);
        }
      }
    } catch (err) {
      console.error(`Failed to check ${platform} auth status:`, err);
    }
  };

  const refreshAuth = async (platform: string): Promise<void> => {
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`http://127.0.0.1:3001/api/auth/${platform}/refresh`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ user_id: 'current_user' }),
      });
      
      if (response.ok) {
        await checkAuthStatus(platform);
      } else {
        throw new Error(`Failed to refresh ${platform} token`);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to refresh ${platform} authentication`;
      setError(errorMessage);
      console.error('Token refresh error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async (platform: string): Promise<void> => {
    setIsLoading(true);
    
    try {
      const response = await fetch(`http://127.0.0.1:3001/api/auth/${platform}/logout`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ user_id: 'current_user' }),
      });
      
      // Clear local auth status regardless of server response
      switch (platform) {
        case 'google':
          setGoogle(defaultAuthStatus);
          break;
        case 'slack':
          setSlack(defaultAuthStatus);
          break;
        case 'github':
          setGithub(defaultAuthStatus);
          break;
        case 'confluence':
          setConfluence(defaultAuthStatus);
          break;
      }
      
      setError(null);
    } catch (err) {
      console.error(`Failed to logout from ${platform}:`, err);
      // Still clear local status even if server logout failed
      switch (platform) {
        case 'google':
          setGoogle(defaultAuthStatus);
          break;
        case 'slack':
          setSlack(defaultAuthStatus);
          break;
        case 'github':
          setGithub(defaultAuthStatus);
          break;
        case 'confluence':
          setConfluence(defaultAuthStatus);
          break;
      }
    } finally {
      setIsLoading(false);
    }
  };

  const logoutAll = async (): Promise<void> => {
    setIsLoading(true);
    
    try {
      const platforms = ['google', 'slack', 'github', 'confluence'];
      await Promise.all(platforms.map(platform => logout(platform)));
      
      // Clear all localStorage
      localStorage.removeItem('platform_auth_status');
      
      setError(null);
    } catch (err) {
      console.error('Failed to logout from all platforms:', err);
      setError('Failed to logout from all platforms');
    } finally {
      setIsLoading(false);
    }
  };

  const isAnyAuthenticated = (): boolean => {
    return google.isAuthenticated || slack.isAuthenticated || 
           github.isAuthenticated || confluence.isAuthenticated;
  };

  const getAuthenticatedPlatforms = (): string[] => {
    const authenticated = [];
    if (google.isAuthenticated) authenticated.push('google');
    if (slack.isAuthenticated) authenticated.push('slack');
    if (github.isAuthenticated) authenticated.push('github');
    if (confluence.isAuthenticated) authenticated.push('confluence');
    return authenticated;
  };

  const value: AuthContextType = {
    google,
    slack,
    github,
    confluence,
    initiateAuth,
    refreshAuth,
    logout,
    logoutAll,
    isAnyAuthenticated,
    getAuthenticatedPlatforms,
    isLoading,
    error,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

export default AuthContext;