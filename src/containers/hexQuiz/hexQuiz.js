import React from 'react';
import HexQuiz from 'components/hexQuiz/hexQuiz';

import './hexQuiz.scss';

export default class HexQuizContainer extends React.Component {
    render() {
        return (
            <div className='hex-quiz-container'>
                <div className='hex-quiz-description'>
                    <h3>Press the corresponding hex for each byte.</h3>
                </div>
                <div><HexQuiz/></div>
            </div>
        );
    }
}
