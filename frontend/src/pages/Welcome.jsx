import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import "./Welcome.css";

const WelcomePage = () => {
  const [visibleSections, setVisibleSections] = useState({});
  const navigate = useNavigate();

  useEffect(() => {
    const sections = ["header", "button", "footer"];
    let index = 0;

    const intervalId = setInterval(() => {
      if (index < sections.length) {
        const section = sections[index];
        setVisibleSections((prev) => ({ ...prev, [section]: true }));
        index++;
      } else {
        clearInterval(intervalId);
      }
    }, 1000);

    return () => clearInterval(intervalId);
  }, []);

  const WelcomeButtonClicked = () => {
    navigate("/signin");
  };

  return (
    <div className="welcome-page">

      {/* Header Section */}
      <Header visible={visibleSections.header} />

      {/* Main Section */}
      <main className={`section main-content ${visibleSections.button ? "visible" : ""}`}>
        <button className="play-button" onClick={WelcomeButtonClicked}>
          Start Playing
        </button>
      </main>

      {/* Footer Section */}
      <Footer visible={visibleSections.footer} />
      
    </div>
  );
};

export default WelcomePage;

// Header Component
const Header = React.memo(({ visible }) => (
  <header className={`section ${visible ? "visible" : ""}`}>
    <h1>Welcome to Chess Play</h1>
    <div className="description">
      <p>
        Experience the classic game of chess with our interactive platform. Whether you're a beginner or a seasoned
        player, our site offers a variety of features to enhance your chess skills.
      </p>
    </div>
  </header>
));

// Footer Component
const Footer = React.memo(({ visible }) => (
  <footer className={`section ${visible ? "visible" : ""}`}>
    <p>© 2023 Chess Play. All rights reserved.</p>
  </footer>
));

