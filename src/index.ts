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
    indexStack = 0;
    counter = 0;

    rows: Set<number>[] = [];
    cols: Set<number>[] = [];
    quadrant: Set<number>[] = [];

    constructor() {
        this.rows = Array(9).fill(null).map(() => new Set());
        this.cols = Array(9).fill(null).map(() => new Set());
        this.quadrant = Array(9).fill(null).map(() => new Set());
        this.board = Array(9*9).fill(0);
    }

    drawBoard() {
        let board = this.board;
        const boardMap: string[] = [];
        boardMap.push(`counter: ${this.counter}`);
        // boardMap.push(`rows: ${this.rows.map((r) => Array.from(r).join(',')).join('|')}`);
        // boardMap.push(`cols: ${this.cols.map((r) => Array.from(r).join(',')).join('|')}`);
        // boardMap.push(`quadrant: ${this.quadrant.map((r) => Array.from(r).join(',')).join('|')}`);
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

        // this.drawDebug();
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

    getPossibleNumbersByIndex(index: number): number[] {
        return [1, 2, 3, 4, 5, 6, 7, 8, 9].filter((n) => {
            if (this.rows[this.getLine(index)].has(n)) {
                return false;
            }
            if (this.cols[this.getColumn(index)].has(n)) {
                return false;
            }
            const indexQuadrant = this.getQuadrant(index);
            return !this.quadrant[indexQuadrant].has(n);

        });
    }

    createNumberRecursive(index: number = 0) {
        if (index >= 81) {
            return true;
        }
        this.drawBoard();

        const listOfIndexes: number[] = this.getPossibleNumbersByIndex(index).sort(() => Math.random() - 0.5);
        while(listOfIndexes.length > 0) {
            this.board[index] = listOfIndexes.pop() as number;
            this.counter++;
            this.setNumberToColumnLineQuadrant(index, this.board[index]);
            if (this.createNumberRecursive(index + 1)) {
                return true;
            }
            this.removeNumberFromColumnLineQuadrant(index, this.board[index]);
        }
    }

    createNumber() {
        const index = this.indexStack;

        let possibleNumbers: number[] = [];
        if (this.stack[index] && Array.isArray(this.stack[index]) && this.stack[index].length > 0) {
            this.removeNumberFromColumnLineQuadrant(index, this.board[index]);
            this.stack[index] = this.stack[index].filter((n) => n !== this.board[index]);
            possibleNumbers = this.stack[index];
        } else {
            possibleNumbers = this.getPossibleNumbersByIndex(index);
        }

        if (possibleNumbers.length === 0) {
            const currentValue = this.board[index];
            this.removeNumberFromColumnLineQuadrant(index, currentValue);
            this.board[index] = 0;
            delete this.stack[index];
            this.indexStack -= 1;
            return;
        }

        const randomNumber = possibleNumbers[Math.floor(Math.random() * possibleNumbers.length)];

        this.stack[index] = possibleNumbers;
        this.board[index] = randomNumber;
        this.setNumberToColumnLineQuadrant(index, randomNumber);
        this.indexStack += 1;
    }

    getLine(index: number) {
        return Math.floor(index / 9);
    }

    getColumn(index: number) {
        return index % 9;
    }

    getQuadrant(index: number) {
        const line = this.getLine(index);
        const column = this.getColumn(index);

        return Math.floor(line / 3) * 3 + Math.floor(column / 3);
    }

    setNumberToColumnLineQuadrant(index: number, number: number) {
        const line = this.getLine(index);
        const column = this.getColumn(index);

        this.cols[column].add(number);
        this.rows[line].add(number);
        const quadrantIndex = this.getQuadrant(index);
        this.quadrant[quadrantIndex].add(number);
    }

    removeNumberFromColumnLineQuadrant(index: number, number: number) {
        const line = this.getLine(index);
        const column = this.getColumn(index);

        this.cols[column].delete(number);
        this.rows[line].delete(number);
        const quadrantIndex = this.getQuadrant(index);
        this.quadrant[quadrantIndex].delete(number);
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
        this.createNumberRecursive();
        // while(true) {
        //     this.createNumber();
        //     this.counter++;
        //     this.drawBoard();
        //
        //     if (this.isFinished()) {
        //         break;
        //     }
            // sleepSync(30);
        // }
    }
}

const board = new Board();
board.generate();
