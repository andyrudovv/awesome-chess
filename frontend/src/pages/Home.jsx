import React from 'react';
import { FaChess, FaChartBar, FaHistory, FaPuzzlePiece, FaUser, FaCog } from 'react-icons/fa';
import { Link } from 'react-router-dom';

import ChessBoard from '../components/ChessBoard';

const Home = () => {
  return (
    <div className="home-page">
      <main className="main-content">
      <aside className="sidebar">
        <div className="sidebar-top">
          <Link to="/play" className="sidebar-icon">
            <FaChess />
            <span>Play</span>
          </Link>
          <Link to="/analyze" className="sidebar-icon">
            <FaChartBar />
            <span>Analyze</span>
          </Link>
          <Link to="/history" className="sidebar-icon">
            <FaHistory />
            <span>History</span>
          </Link>
          <Link to="/puzzles" className="sidebar-icon">
            <FaPuzzlePiece />
            <span>Puzzles</span>
          </Link>
        </div>
        <div className="sidebar-bottom">
          <Link to="/profile" className="sidebar-icon">
            <FaUser />
            <span>Profile</span>
          </Link>
          <Link to="/settings" className="sidebar-icon">
            <FaCog />
            <span>Settings</span>
          </Link>
        </div>
      </aside>
        <ChessBoard/>
      </main>
    </div>
  );
};

export default Home;
