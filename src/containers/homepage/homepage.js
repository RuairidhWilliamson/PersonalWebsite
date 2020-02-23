import React from 'react';
import { Helmet } from 'react-helmet';

import Button from 'components/button/button';

import './homepage.scss';

export default class Homepage extends React.Component {
    render() {
        return <>
            <Helmet>
                <title>Ruairidh Williamson</title>
                <meta name='description' content=''/>
            </Helmet>
            <div className='homepage'>
                <div className='middle'>
                    <h1>Ruairidh Williamson</h1>
                    <br/>
                    <h2>I am currently looking for a summer internship for 2020.</h2>
                    <br/>
                    <div>
                        <Button to='/projects'>Projects</Button>
                        <Button to='/contact'>Contact</Button>
                    </div>
                </div>
            </div>
        </>;
    }
}