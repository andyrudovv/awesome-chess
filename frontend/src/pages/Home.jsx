import React, { useState } from "react";
import "./Home.css";
import Sidebar from "../components/Sidebar";
import Board from "../components/Board/Board";

const HomePage = () => {
  const [activeNav, setActiveNav] = useState("home");
  const [engineSettings, setEngineSettings] = useState({
    engine: "Stockfish",
    depth: 10,
  });

  const handleSettingsChange = (e) => {
    const { name, value } = e.target;
    setEngineSettings((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  return (
    <div className="home-page">
      <Sidebar activeNav={activeNav} setActiveNav={setActiveNav} />
      <div className="main-content">
        {/* Settings Section */}
        <div className="settings-field">
          <h3>Engine Settings</h3>
          <label>
            Engine:
            <select
              name="engine"
              value={engineSettings.engine}
              onChange={handleSettingsChange}
            >
              <option value="Stockfish">Stockfish</option>
              <option value="Mine">Mine</option>
            </select>
          </label>
          <label>
            Depth:
            <input
              type="number"
              name="depth"
              value={engineSettings.depth}
              onChange={handleSettingsChange}
              min="1"
              max="30"
            />
          </label>
        </div>
        
        {/* Chessboard Section */}
        <Board />
      </div>
    </div>
  );
};

export default HomePage;
