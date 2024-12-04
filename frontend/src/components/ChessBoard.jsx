import React from 'react';
import { Chessboard } from 'react-chessboard';

const ChessBoard = () => {
  return (
    <div className="chess-board-container" style={{height:'500px', width:'500px'}}>
      <Chessboard />
    </div>
  );
};

export default ChessBoard;