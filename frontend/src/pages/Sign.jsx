import React, { useState } from "react";
import { FaGoogle, FaGithub } from "react-icons/fa";
import "./Sign.css";

const Sign = () => {
  const [isLogin, setIsLogin] = useState(true); // Track if it's login or sign up
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState(""); // State for confirm password

  const handleSubmit = (e) => {
    e.preventDefault();

    if (!isLogin && password !== confirmPassword) {
      console.log("Passwords do not match!");
      return; // Optionally, show an error message to the user
    }

    // Handle email/password registration or login
    console.log(isLogin ? "Logging in" : "Signing up", email, password);
  };

  const handleGoogleAuth = () => {
    // Handle Google authentication
    console.log("Authenticating with Google");
  };

  const handleGithubAuth = () => {
    // Handle GitHub authentication
    console.log("Authenticating with GitHub");
  };

  return (
    <div className="sign-page">
      <div className="sign-container">
        <h2>{isLogin ? "Login" : "Sign Up"}</h2>
        <form onSubmit={handleSubmit}>
          <input
            type="email"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
          <input
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
          {!isLogin && (
            <input
              type="password"
              placeholder="Confirm Password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              required={!isLogin}
            />
          )}
          <button type="submit">{isLogin ? "Login" : "Sign Up"}</button>
        </form>
        <div className="auth-options">
          <button onClick={handleGoogleAuth}>
            <FaGoogle /> Google
          </button>
          <button onClick={handleGithubAuth}>
            <FaGithub /> GitHub
          </button>
        </div>
        <p>
          {isLogin ? "Don't have an account? " : "Already have an account? "}
          <span onClick={() => setIsLogin(!isLogin)}>
            {isLogin ? "Sign Up" : "Log in"}
          </span>
        </p>
      </div>
    </div>
  );
};

export default Sign;

