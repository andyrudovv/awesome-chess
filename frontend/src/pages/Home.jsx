import React, { useState } from "react";
import { FaHome, FaChess, FaUser, FaCog } from "react-icons/fa";
import ChessBoard from "../components/ChessBoard";
import "./Home.css";

const HomePage = () => {
  const [activeNav, setActiveNav] = useState("home");
  const [engineSettings, setEngineSettings] = useState({
    engine: "Stockfish",
    depth: 10,
  });

  const topNavItems = [
    { icon: <FaHome />, key: "home" },
    { icon: <FaChess />, key: "play" },
  ];

  const bottomNavItems = [
    { icon: <FaUser />, key: "profile" },
    { icon: <FaCog />, key: "settings" },
  ];

  const handleSettingsChange = (e) => {
    const { name, value } = e.target;
    setEngineSettings((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const renderNavItems = (items) =>
    items.map((item) => (
      <li
        key={item.key}
        className={activeNav === item.key ? "active" : ""}
        onClick={() => setActiveNav(item.key)}
      >
        {item.icon}
      </li>
    ));

  return (
    <div className="home-page">
      <nav className="sidebar">
        <div className="sidebar-top">
          <ul>{renderNavItems(topNavItems)}</ul>
        </div>
        <div className="sidebar-bottom">
          <ul>{renderNavItems(bottomNavItems)}</ul>
        </div>
      </nav>
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
        <ChessBoard />
      </div>
    </div>
  );
};

export default HomePage;
