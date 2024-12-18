import React from "react";
import { FaHome, FaChess, FaUser, FaCog } from "react-icons/fa";
import { useNavigate } from "react-router-dom";

const Sidebar = ({ activeNav, setActiveNav }) => {

    const navigate = useNavigate();

    const onHomeButtonClick = () => {
        setActiveNav("home")
        navigate("/home");
    }

    const onPlayButtonClick = () => {
        setActiveNav("play")
        navigate("/play");
    }

    const onProfileButtonClick = () => {
        setActiveNav("profile")
        navigate("/profile");
    }

    const onSettingsButtonClick = () =>
    {
        setActiveNav("settings")
        navigate("/settings");
    }
  
  return (
    <nav className="sidebar">
      <div className="sidebar-top">
        <ul>
          <li
            className={activeNav === "home" ? "active" : ""}
            onClick={() => onHomeButtonClick() }
          >
            <FaHome />
          </li>
          <li
            className={activeNav === "play" ? "active" : ""}
            onClick={() => onPlayButtonClick()}
          >
            <FaChess />
          </li>
        </ul>
      </div>
      <div className="sidebar-bottom">
        <ul>
          <li
            className={activeNav === "profile" ? "active" : ""}
            onClick={() => onProfileButtonClick()}
          >
            <FaUser />
          </li>
          <li
            className={activeNav === "settings" ? "active" : ""}
            onClick={() => onSettingsButtonClick()}
          >
            <FaCog />
          </li>
        </ul>
      </div>
    </nav>
  );
};

export default Sidebar;
