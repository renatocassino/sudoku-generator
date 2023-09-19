const term = require( 'terminal-kit' ).terminal ;

function sleepSync(milliseconds: number) {
    const date = Date.now();
    let currentDate: number | null = null;
    do {
        currentDate = Date.now();
    } while (currentDate - date < milliseconds);
}

class Board {
    board: number[] = [];
    stack: number[][] = [];
    index = 0;
    counter = 0;
    drawer: any;

    constructor() {
        for (let i = 0; i < 9*9; i++) {
            this.board.push(0);
        }
    }

    drawBoard() {
        let board = this.board;
        const boardMap: string[] = [];
        boardMap.push(`counter: ${this.counter}`);
        boardMap.push('');

        let line: string[] = [];
        const horizontalLine = ' ------+-------+------ ';

        for (let i = 0; i < board.length; i++) {

            if (i % 9 === 0 && i > 0) {
                boardMap.push(` ${line.join(' ')} `);
                line  = [];
            } else if (i % 3 === 0 && i > 0) {
                line.push("|");
            }
            line.push(board[i].toString());


            if (i === 27 || i === 54) {
                boardMap.push(horizontalLine);
            }
        }
        boardMap.push(` ${line.join(' ')} `);

        term.moveTo( 0, 0 );
        term(boardMap.join(`\n`));

        this.drawDebug();
    }

    drawDebug() {
        let line: string[] = [];
        const stack = this.stack;
        const boardMap: string[] = [];
        const horizontalLine = ' ------+-------+------ ';

        term('\n\n\n');
        for (let i = 0; i < 81; i++) {

            if (i % 9 === 0 && i > 0) {
                boardMap.push(` ${line.join(' ')} `);
                line  = [];
            } else if (i % 3 === 0 && i > 0) {
                line.push("|");
            }

            if (i in stack) {
                line.push(stack[i].length.toString());
            } else {
                line.push('0');
            }

            if (i === 27 || i === 54) {
                boardMap.push(horizontalLine);
            }
        }
        boardMap.push(` ${line.join(' ')} `);

        term(boardMap.join(`\n`));
    }

    getColumn(index: number) {
        let column: number[] = [];
        for (let i = index - 9; i > 0; i -= 9) {
            if (this.board[i] === 0) {
                continue;
            }
            column.push(this.board[i]);
        }
        for (let i = index + 9; i < this.board.length; i += 9) {
            if (this.board[i] === 0) {
                continue;
            }
            column.push(this.board[i]);
        }

        return column;
    }

    getLine(index: number) {
        let line: number[] = [];
        let start = index - (index % 9);
        for (let i = start; i < start + 9; i++) {
            if (this.board[i] === 0) {
                continue;
            }
            line.push(this.board[i]);
        }
        return line;
    }

    getQuadrantNumber(index: number): [number, number] {
        let quadrantX = Math.floor(index / 3) % 3;
        let quadrantY = Math.floor(Math.floor(index / 9) / 3);

        return [quadrantX, quadrantY];
    }

    getQuadrantByIndex(index: number): number[] {
        let quadrant: number[] = [];
        let [quadrantX, quadrantY] = this.getQuadrantNumber(index);

        let start = (quadrantY * 27) + (quadrantX * 3);
        for (let i = start; i < start + 3; i++) {
            if (this.board[i] === 0) {
                continue;
            }
            quadrant.push(this.board[i]);
        }
        for (let i = start + 9; i < start + 12; i++) {
            if (this.board[i] === 0) {
                continue;
            }
            quadrant.push(this.board[i]);
        }
        for (let i = start + 18; i < start + 21; i++) {
            if (this.board[i] === 0) {
                continue;
            }
            quadrant.push(this.board[i]);
        }

        return quadrant;
    }

    getPossibleNumbersByIndex(index: number): number[] {
        const numbers = [...this.getColumn(index), ...this.getLine(index), ...this.getQuadrantByIndex(index)];

        const allNumbers = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        numbers.forEach((number) => {
            allNumbers.delete(number);
        });

        return [...allNumbers];
    }

    createNumber() {
        const index = this.index;

        let possibleNumbers: number[] = [];
        if (this.stack[index] && Array.isArray(this.stack[index]) && this.stack[index].length > 0) {
            this.stack[index] = this.stack[index].filter((n) => n !== this.board[index]);
            possibleNumbers = this.stack[index];
        } else {
            possibleNumbers = this.getPossibleNumbersByIndex(index);
        }

        if (possibleNumbers.length === 0) {
            this.board[index] = 0;
            this.index -= 1;
            return;
        }

        const randomNumber = possibleNumbers[Math.floor(Math.random() * possibleNumbers.length)];

        this.stack[index] = possibleNumbers;
        this.board[index] = randomNumber;
        this.index += 1;
    }

    isFinished() {
        return this.board[this.board.length - 1] !== 0;
    }

    clearTerminal() {
        term.moveTo(0, 0);

        for(let i = 0; i < 30; i++) {
            console.log('                                       ');
        }
    }

    generate() {
        this.clearTerminal();
        while(true) {
            this.createNumber();
            this.counter++;
            this.drawBoard();

            if (this.isFinished()) {
                break;
            }
            // sleepSync(30);
        }
    }

    generateOld() {
        board.drawBoard();

        for (let i = 0; i < 9*9; i++) {
            const possibleNumbers = this.getPossibleNumbersByIndex(i);
            const randomNumber = possibleNumbers[Math.floor(Math.random() * possibleNumbers.length)];
            this.board[i] = randomNumber;
            board.drawBoard();
        }
    }
}

const board = new Board();
board.generate();
