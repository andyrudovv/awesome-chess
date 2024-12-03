import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import './Welcome.css';

const HomePage = () => {
    const [visibleSections, setVisibleSections] = useState([]);
    const navigate = useNavigate();

    useEffect(() => {
        const sections = ['header', 'description', 'button', 'footer'];
        let index = 0;

        const intervalId = setInterval(() => {
            if (index < sections.length) {
                setVisibleSections(prev => [...prev, sections[index]]);
                index++;
            } else {
                clearInterval(intervalId);
            }
        }, 1000); // Delay of 1 second between each section

        return () => clearInterval(intervalId);
    }, []);

    const WelcomeButtonClicked = () => {
        navigate('/signin'); // Redirect to SignIn page
    };

    return (
        <div className="home-page">
            <header className={`section ${visibleSections.includes('header') ? 'visible' : ''}`}>
                <h1>Welcome to Chess Play</h1>

                <div className={`description`}>
                    <p>
                        Experience the classic game of chess with our interactive platform. Whether you're a beginner or a seasoned player, our site offers a variety of features to enhance your chess skills.
                    </p>
                </div>
            </header>

            

            <main className={`section main-content ${visibleSections.includes('button') ? 'visible' : ''}`}>
                
                <button
                    className="play-button"
                    onClick={() => WelcomeButtonClicked()}
                >
                    Start Playing
                </button>
            </main>

            <footer className={`section ${visibleSections.includes('footer') ? 'visible' : ''}`}>
                <p>© 2023 Chess Play. All rights reserved.</p>
            </footer>
        </div>
    );
};

export default HomePage;
