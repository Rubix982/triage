# 🕸️ Knowledge Graph Setup Guide

## 🚀 Quick Start

The visual knowledge graph feature is now implemented! Here's how to run it:

### 1. Set up Jira Authentication & Data
```bash
# Navigate to server
cd server

# Install dependencies (if needed)
cargo build

# Authenticate with your Jira instance
cargo run login

# Fetch and sync project data
cargo run projects
```

### 2. Start the Backend Server
```bash
# Start the API server (serves graph data)
cargo run serve
```
The server will start at `http://127.0.0.1:3001`

### 3. Start the Frontend Client
```bash
# Navigate to client (in new terminal)
cd ../client

# Install dependencies (if needed)
pnpm install

# Start the Electron app with hot reload
pnpm dev
```

## 🎯 Features Implemented

### **Interactive Knowledge Graph**
- **Force-directed network visualization** using D3.js
- **Drag & drop nodes** to explore relationships
- **Zoom & pan** for navigation
- **Click nodes** for detailed information
- **Hover tooltips** with quick info

### **Smart Filtering & Search**
- **Search bar**: Find nodes by name, type, or key
- **Node type filter**: Show only Issues, Projects, etc.
- **Max nodes limit**: Control graph complexity (25-200 nodes)
- **Real-time filtering**: Instant updates without server calls
- **Clear filters button**: Reset all filters quickly

### **Rich Node Types**
- **Issues**: Color-coded by status (Green=Done, Yellow=In Progress, etc.)
- **Projects**: Larger nodes showing project relationships
- **Future**: People, Technologies, Concepts (framework ready)

### **Intelligent Relationships**
- **Part Of**: Issues belong to projects
- **References**: Issue links and dependencies  
- **Future**: Similar issues, collaborations, mentions

### **Professional UI**
- **Dark theme** with modern design
- **Tabbed interface**: Switch between Graph and Analytics
- **Responsive layout**: Works on different screen sizes
- **Loading states** and error handling

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Jira API      │───▶│   Rust Backend   │───▶│  Electron UI    │
│                 │    │                  │    │                 │
│ • Projects      │    │ • DuckDB Storage │    │ • React + D3.js │
│ • Issues        │    │ • Graph Analysis │    │ • Interactive   │
│ • Metadata      │    │ • REST API       │    │ • Filtering     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### **Backend (Rust)**
- **Data Processing**: Extracts relationships from Jira metadata
- **Graph Generation**: Creates nodes and edges from issue data
- **API Endpoints**: 
  - `GET /api/graph` - Returns knowledge graph data
  - `GET /api/graph/analysis` - Returns graph analytics
  - `GET /health` - Health check

### **Frontend (TypeScript + React)**
- **D3.js Visualization**: Force-directed graph with physics simulation
- **Interactive Controls**: Search, filter, zoom, drag
- **Real-time Updates**: Filters apply instantly without server round-trips

## 📊 What You'll See

### **Typical Graph Elements**
1. **Project Nodes** (Large, Indigo): Your Jira projects
2. **Issue Nodes** (Color-coded): 
   - 🟢 Green: Completed issues
   - 🟡 Yellow: In progress
   - 🟣 Purple: In review  
   - 🔴 Red: Open/blocked
3. **Edges**: Relationships between issues and projects

### **Interactive Features**
- **Drag nodes** to rearrange the layout
- **Zoom with mouse wheel** or pinch gesture
- **Click nodes** to see detailed metadata in side panel
- **Search** to highlight specific nodes
- **Filter by type** to focus on specific elements

## 🔧 Customization Options

### **Backend Configuration**
```rust
// In server/src/graph.rs - modify these to change graph behavior:

// Limit issues fetched (adjust for performance)
"SELECT id, key, summary, status, project FROM issues LIMIT 100"

// Node sizing logic
let size = match status.as_str() {
    "Done" | "Closed" => 8.0,
    "In Progress" | "In Review" => 12.0,
    _ => 10.0,
};

// Color schemes
let color = match status.as_str() {
    "Done" | "Closed" => "#10B981", // Green
    "In Progress" => "#F59E0B",     // Yellow
    // ... add your custom colors
};
```

### **Frontend Configuration**
```typescript
// In client/src/components/KnowledgeGraph.tsx

// Graph physics parameters
.force('charge', d3.forceManyBody().strength(d => -100 - d.size * 10))
.force('link', d3.forceLink().distance(d => 50 + d.weight * 20))

// Visual styling
.attr('r', d => d.size)              // Node size
.attr('stroke-width', 2)             // Border width
.attr('font-size', d => d.size / 3)  // Label size
```

## 🚀 Next Steps & Enhancements

The foundation is built! Ready for the advanced features from `ENHANCEMENT_IDEAS.md`:

### **Phase 2** (Next priorities)
1. **NLP Content Analysis**: Extract technologies/concepts from issue descriptions
2. **Semantic Search**: Find similar issues using AI embeddings  
3. **Learning Analytics**: Track skill progression over time
4. **Advanced Relationships**: People, technologies, code components

### **Phase 3** (Advanced features)
1. **AI-Powered Insights**: Automated pattern recognition
2. **Real-time Collaboration**: Live updates, shared views
3. **Export & Reporting**: Generate insights for broader team
4. **Integration Ecosystem**: Connect with other dev tools

## 🐛 Troubleshooting

### **Common Issues**

**"Failed to fetch graph data"**
- Ensure backend server is running (`cargo run serve`)
- Check server is accessible at `http://127.0.0.1:3001`
- Verify you have issues data (`cargo run projects` first)

**"Empty graph"**
- Run `cargo run projects` to sync Jira data
- Check DuckDB has data: look for `issues` table
- Try increasing max nodes limit in UI

**"Graph too cluttered"**  
- Reduce max nodes (try 25-50)
- Use search/filters to focus on specific areas
- Drag nodes to organize layout manually

**"Performance issues"**
- Lower max nodes limit
- Consider upgrading hardware for large datasets
- Future: implement virtual rendering for huge graphs

## 📈 Metrics & Success

You now have the foundation for **accelerated learning**! Track these metrics:

- **Discovery Time**: How quickly you find relevant past work
- **Context Acquisition**: Speed of understanding project relationships  
- **Knowledge Reuse**: Frequency of applying past solutions
- **Team Collaboration**: Better understanding of who worked on what

Ready to revolutionize how your team learns and collaborates! 🚀