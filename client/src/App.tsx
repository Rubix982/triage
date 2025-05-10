import React from 'react'
import MyChart from './components/MyChart'

export default function App() {
  return (
    <div className="min-h-screen bg-gray-900 p-8 text-white">
      <h1 className="mb-6 text-2xl font-bold">D3 + React + Electron</h1>
      <MyChart />
    </div>
  )
}
