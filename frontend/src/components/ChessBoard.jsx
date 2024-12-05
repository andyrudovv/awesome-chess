import React from "react";
import { Chessboard } from "react-chessboard";

const ChessBoard = () => {
  return (
    <div className="chess-board">
      <Chessboard id="BasicBoard" />
    </div>
  );
};

export default ChessBoard;