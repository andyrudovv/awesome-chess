export const getCharacter = file => String.fromCharCode(file + 96)


export const createPosition = () => {
    const position = new Array(8).fill('').map(x => new Array(8).fill(''))

    position[0][0] = 'rook-w'
    position[0][7] = 'rook-w'
    position[7][0] = 'rook-b'
    position[7][7] = 'rook-b'

    position[0][1] = 'knight-w'
    position[0][6] = 'knight-w'
    position[7][6] = 'knight-b'
    position[7][1] = 'knight-b'

    position[0][2] = 'bishop-w'
    position[0][5] = 'bishop-w'
    position[7][5] = 'bishop-b'
    position[7][2] = 'bishop-b'

    position[7][3] = 'queen-b'
    position[0][3] = 'queen-w'

    position[7][4] = 'king-b'
    position[0][4] = 'king-w'

    for (let i = 0; i < 8; i++) {
        position[1][i] = 'pawn-w'
        position[6][i] = 'pawn-b'
    }

    return position;
}

export const copyPosition = position => {
    const newPosition = new Array(8).fill('').map(x => new Array(8).fill(''))

    for (let rank = 0; rank < 8; rank++) {
        for (let file = 0; file < 8; file++) {
            newPosition[rank][file] = position[rank][file]
        }
    }

    return newPosition;
}