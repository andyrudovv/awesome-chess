import React, { useState } from "react";
import Sidebar from "../components/Sidebar";
import "./Play.css";

const PlayPage = () => {
  const [activeNav, setActiveNav] = useState("play");


  return (
    <div className="play-page">
      <Sidebar activeNav={activeNav} setActiveNav={setActiveNav} />
      <div className="main-content">
        
      </div>
    </div>
  );
};

export default PlayPage;
