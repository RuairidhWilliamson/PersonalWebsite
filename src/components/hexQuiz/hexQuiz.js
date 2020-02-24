import React from 'react';
import { Button } from 'react-bootstrap';

import './hexQuiz.scss';

const randomBinary = n => [...Array(n)].map(() => Math.random() < 0.5 ? '0' : '1').join('');

const convertHex = x => hex[Array.prototype.map.call(x, el => el).map((c, i, arr) => Math.pow(2, arr.length - i - 1) * parseInt(c)).reduce((total, val) => total + val, 0)];

const hex = '0123456789abcdef';

export default class HexQuiz extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            binary: '',
            input: '',
            correct: 0,
            time: 0,
        };
        this.startTime = 0;
        document.addEventListener('keypress', this.handleKey.bind(this));
    }

    componentWillUnmount() {
        document.removeEventListener('keypress', this.handleKey.bind(this));
    }

    genNewBinary() {
        this.setState({
            binary: randomBinary(4),
        });
    }

    handleStart() {
        if (!this.state.binary) {
            this.startTime = new Date();
            this.setState({
                correct: 0,
                time: 0,
            });
            this.genNewBinary();
        }
    }

    handleKey(ev) {
        if (hex.includes(ev.key)) {
            const correct = convertHex(this.state.binary);
            if (correct === ev.key) {
                this.setState(state => ({
                    time: (new Date() - this.startTime) / 1000,
                    correct: state.correct + 1,
                }));
                this.genNewBinary();
            } else {
                this.setState({
                    binary: '',
                });
            }
        }
    }

    render() {
        const keypad = [...Array(hex.length)].map((_, i) => <Button className="m-1" onClick={() => this.handleKey({key: hex[i]})} key={i}>{hex[i]}</Button>);
        return <div className='hex-quiz'>
            <div className='binary'>{this.state.binary}</div>
            <div className='hex-quiz__input'>{this.state.input}</div>
            <div>{!this.state.binary && <Button onClick={this.handleStart.bind(this)}>Start</Button>}</div>
            <br/>
            <div>
                {this.state.correct} correct -  avg time {!this.state.correct ? 0 : Math.round(this.state.time / this.state.correct * 1000) / 1000}s
            </div>
            {this.state.binary &&
                <div className='hex-quiz__keypad'>
                    {keypad}
                </div>
            }
        </div>;
    }
}
