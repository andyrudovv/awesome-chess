import './Board.css'

import Ranks from './bits/Ranks'
import Files from './bits/Files'
import Pieces from '../Pieces/Pieces'

const Board = () => {
    const ranks = Array(8).fill().map((x, i) => 8 - i)
    const files = Array(8).fill().map((x, i) => i+1)


    const getClassName = (i, j) => {
        let _class = 'tile'
        _class += (i+j) % 2 === 0? ' tile--dark' : ' tile--light';

        return _class;
    }

    return (
        <div className='board'>
            <Ranks ranks={ranks}/>
            <div className='tiles'>
                {ranks.map((rank, i) =>
                    files.map((file, j) =>
                        <div key={file+'-'+rank} className={getClassName(9-i, j)}></div>
                    )
                )}
            </div>

            <Pieces />

            <Files files={files}/>
        </div>
    )
}

export default Board;