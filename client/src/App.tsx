import React, { useState } from 'react'
import { AuthProvider } from './contexts/AuthContext'
import AdvancedDashboard from './components/AdvancedDashboard'
import AnalyticsDashboard from './components/AnalyticsDashboard'
import KnowledgeGraph from './components/KnowledgeGraph'
import KnowledgeBase from './components/KnowledgeBase'
import SmartSearch from './components/SmartSearch'
import UniversalSearch from './components/UniversalSearch'
import SyncStatusDashboard from './components/SyncStatusDashboard'
import MyChart from './components/MyChart'
import PeopleIntelligence from './components/PeopleIntelligence'

export default function App() {
  const [activeTab, setActiveTab] = useState<'universal' | 'search' | 'sync' | 'advanced' | 'analytics' | 'graph' | 'knowledge' | 'people' | 'chart'>('universal')

  return (
    <AuthProvider>
      <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header className="border-b border-gray-700 p-4">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-blue-400">🧠 Triage</h1>
            <p className="text-gray-300 mt-1">
              Universal Knowledge Extraction & Search Platform
            </p>
          </div>
          <div className="text-right">
            <p className="text-sm text-gray-400">Cross-Platform Intelligence</p>
            <p className="text-xs text-gray-500">Jira • Google • Slack • GitHub</p>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="border-b border-gray-700 px-4">
        <div className="flex space-x-1">
          <button
            onClick={() => setActiveTab('universal')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'universal'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            🌐 Universal Search
          </button>
          <button
            onClick={() => setActiveTab('search')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'search'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            🔍 Semantic Search
          </button>
          <button
            onClick={() => setActiveTab('sync')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'sync'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            📊 Sync Status
          </button>
          <button
            onClick={() => setActiveTab('advanced')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'advanced'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            🧠 AI Analytics
          </button>
          <button
            onClick={() => setActiveTab('analytics')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'analytics'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            📊 Standard Analytics
          </button>
          <button
            onClick={() => setActiveTab('graph')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'graph'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            🕸️ Knowledge Graph
          </button>
          <button
            onClick={() => setActiveTab('knowledge')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'knowledge'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            🧠 Knowledge Base
          </button>
          <button
            onClick={() => setActiveTab('people')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'people'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            👥 People Intelligence
          </button>
          <button
            onClick={() => setActiveTab('chart')}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 ${
              activeTab === 'chart'
                ? 'text-blue-400 border-blue-400 bg-gray-800'
                : 'text-gray-400 border-transparent hover:text-gray-300'
            }`}
          >
            📈 Basic Charts
          </button>
        </div>
      </nav>

      {/* Main Content */}
      <main className={activeTab === 'people' ? '' : 'p-6'}>
        {activeTab === 'universal' && <UniversalSearch />}
        {activeTab === 'search' && <SmartSearch />}
        {activeTab === 'sync' && <SyncStatusDashboard />}
        {activeTab === 'advanced' && <AdvancedDashboard />}
        {activeTab === 'analytics' && <AnalyticsDashboard />}
        {activeTab === 'graph' && <KnowledgeGraph width={1200} height={700} />}
        {activeTab === 'knowledge' && <KnowledgeBase />}
        {activeTab === 'people' && <PeopleIntelligence />}
        {activeTab === 'chart' && <MyChart />}
      </main>
    </div>
    </AuthProvider>
  )
}
